using System.ComponentModel;
using System.Diagnostics;
using static Global.Admin.AdminFunc;

namespace Global.Admin
{
    public static class FireWall
    {
        /// <summary>
        /// ouvre le port fournit avec le nom fournit l'acces en in et out en protocol TCP
        /// </summary>
        /// <param name="port">le port avec le quel interagir</param>
        /// <param name="RuleName">le nom de la regle</param>
        /// <returns></returns>
        public static string OpenPort(int port, string RuleName = "MonAppPortTCP")
        {
            if (IsAdministrator)
            {
                //ajoute une regle in au firewall
                string commande = $"netsh advfirewall firewall add rule name=\"{RuleName}\" dir=in action=allow protocol=TCP localport={port};";
                //ajoute une regle out au firewall
                commande += $"netsh advfirewall firewall add rule name=\"{RuleName}\" dir=out action=allow protocol=TCP localport={port};";

                ProcessStartInfo psi = new ProcessStartInfo
                {
                    FileName = "powershell",
                    Arguments = $"-Command \"{commande}\"",
                    Verb = "runas", // Nécessite droits admin
                    UseShellExecute = false,
                    CreateNoWindow = true
                };

                try
                {
                    Process.Start(psi);
                    return $"Règle pare-feu ajoutée pour le port {port}";
                }
                catch (Win32Exception e)
                {
                    return "Erreur (l'utilisateur a peut-être refusé les droits admin) : " + e.Message;
                }
            }
            else
            {
                return "l'app n'a pas les perms";
            }
        }

        /// <summary>
        /// affiche toute les regle lier au nom de la regle
        /// </summary>
        /// <param name="name">le nom de la regle</param>
        /// <returns>les infos obtenu</returns>
        public static string ShowRule(string name)
        {
            if (IsAdministrator)
            {
                //string commande = $"New-NetFirewallRule -DisplayName \"{nomRegle}\" -Direction Inbound -Action Allow -Protocol TCP -LocalPort {port}";
                string powershellCommand = $"Get-NetFirewallRule -DisplayName '*{name}*' | ForEach-Object " + "{" + //pour chaque connection trouver
                                           "$rule = $_; " + //stock les premieres infos dans rules
                                           "$portFilter = Get-NetFirewallPortFilter -AssociatedNetFirewallRule $rule; " + //stock la suite des info dans portFilter
                                           "[PSCustomObject]@{ " +
                                               "Name = $rule.DisplayName; " + //affiche le nom
                                               "Direction = $rule.Direction; " + //affiche si in ou out
                                               "Protocol = $portFilter.Protocol; " + //affiche le protocol utiliser
                                               "LocalPort = $portFilter.LocalPort; " + //affiche le port sur le quel il ecoute
                                               "Action = $rule.Action; " + //affiche si il est blocker ou allow
                                               "Enabled = $rule.Enabled " + //affiche si il est actif ou pas
                                           "} " +
                                       "}";

            ProcessStartInfo psi = new ProcessStartInfo
            {
                FileName = "powershell",
                Arguments = $"-Command \"{powershellCommand}\"",
                Verb = "runas", // Nécessite droits admin
                UseShellExecute = false,
                RedirectStandardOutput = true,
                RedirectStandardError = true,
                CreateNoWindow = true
            };

            try
            {
                using (Process process = Process.Start(psi))
                {
                    string output = process.StandardOutput.ReadToEnd();
                    string error = process.StandardError.ReadToEnd();
                    process.WaitForExit();

                    if (!string.IsNullOrWhiteSpace(error))
                    {
                        return error;
                    }

                    return output;
                }
            }
            catch (Win32Exception e)
            {
                return "Erreur (l'utilisateur a peut-être refusé les droits admin) : " + e.Message;
                }
            }
            else
            {
                return "l'app n'a pas les perms";
            }
        }
    }
}