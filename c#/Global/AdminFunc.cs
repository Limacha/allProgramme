using Microsoft.Win32;
using System.Diagnostics;
using System.Reflection;
using System;
using System.Security.Principal;

namespace Global.Admin
{
    public static class AdminFunc
    {
        /// <summary>
        /// verif si l'app a les perms admin
        /// </summary>
        public static bool IsAdministrator { get { return (new WindowsPrincipal(WindowsIdentity.GetCurrent())).IsInRole(WindowsBuiltInRole.Administrator); ; } }

        /// <summary>
        /// fait que lapplication se lance au demarage de la session utilisateur
        /// </summary>
        /// <param name="name">le nom de la registrykey</param>
        /// <param name="path">le chemin del'executable a lancer</param>
        /// <returns></returns>
        public static bool SetLaunchOnStart(string name, string path)
        {
            if (IsAdministrator)
            {
                // Emplacement du registre pour l'utilisateur courant
                RegistryKey rk = Registry.CurrentUser.OpenSubKey(@"Software\Microsoft\Windows\CurrentVersion\Run", true);
                if (name != null && path != null && rk != null)
                {
                    // Ajouter la valeur si elle n'existe pas encore
                    if (rk.GetValue(name) == null)
                    {
                        rk.SetValue(name, "\"" + path + "\"");
                        return true;
                    }
                }
            }
            return false;

        }

        /// <summary>
        /// relance l'app avec les perms admin
        /// </summary>
        /// <param name="exePath">le chemin de l'exe a lancer</param>
        /// <returns>renvoie si reussit ou pas</returns>
        public static bool LaunchAsAdmin(string exePath)
        {
            var startInfo = new ProcessStartInfo(exePath)
            {
                Verb = "runas", // demande élévation UAC
                UseShellExecute = true
            };

            try
            {
                Process.Start(startInfo);
                Environment.Exit(0); // fermer l'ancienne instance
                return true;
            }
            catch
            {
                return false;
            }
        }

        //string exePath = Application.ExecutablePath;
        /// <summary>
        /// ajout l'app au app lancer au demarage de la session
        /// </summary>
        /// <param name="name">le nom du registre</param>
        /// <param name="exePath">le chemin de l'executable</param>
        /// <returns>renvoie si lapp a ette ajouter ou existe deja</returns>
        public static bool SetStart(string name, string exePath)
        {
            if (IsAdministrator)
            {
                // Emplacement du registre pour l'utilisateur courant
                RegistryKey rk = Registry.CurrentUser.OpenSubKey(@"Software\Microsoft\Windows\CurrentVersion\Run", true);

                // Ajouter la valeur si elle n'existe pas encore
                if (rk.GetValue(name) == null)
                {
                    rk.SetValue(name, "\"" + exePath + "\"");
                    return true;
                }
            }
            return false;
        }
    }
}