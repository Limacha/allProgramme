using System;
using System.Collections.Generic;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices.ComTypes;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace RATServeur
{
    public class File : Form
    {
        private Home main;
        private Label pathLbl;
        private Label fileLbl;
        private Panel panelFiles;
        private string currentPath = @"C:\"; // Dossier de départ
        public File(Home mainForm)
        {
            main = mainForm;
            InitializeComponent();
            Init();
            main.GetRemoteFile(currentPath);
        }

        private void Init()
        {
        }

        /// <summary>
        /// affiche tout les fichier dans le string
        /// </summary>
        /// <param name="file">les fichier a ajouter</param>
        public void ShowFile(string file)
        {
            pathLbl.Text = $"path: {currentPath}";


            if (panelFiles.InvokeRequired)
            {
                panelFiles.Invoke(new Action(() =>
                {
                    panelFiles.Controls.Clear();
                }));
            }
            else
            {
                panelFiles.Controls.Clear();
            }

            string[] entries = file.Split("|");

            AddEntriesToPanel(entries, panelFiles);

            fileLbl.Text = file;
            fileLbl.Location = new Point(this.ClientSize.Width, 0);

            AddItemToControls(fileLbl, panelFiles);

        }

        /// <summary>
        /// ajoute tout les flow panel des dir/file
        /// </summary>
        /// <param name="entries">tout les element a ajouter</param>
        /// <param name="mainPanel">le panel a qui tout ajouter</param>
        private void AddEntriesToPanel(string[] entries, Panel mainPanel)
        {
            int yPosition = 10; // Position verticale initiale

            foreach (string entry in entries)
            {
                if (entry.Contains('[') && entry.Contains(']'))
                {
                    FlowLayoutPanel itemPanel = new FlowLayoutPanel
                    {
                        Size = new Size(this.ClientSize.Width - 30, 50),
                        Location = new Point(10, yPosition),
                        BackColor = Color.LightGray,
                        BorderStyle = BorderStyle.FixedSingle
                    };

                    // PictureBox (icône fichier/dossier)
                    PictureBox pictureBox = new PictureBox
                    {
                        Size = new Size(40, 40),
                        Location = new Point(5, 5),
                        Image = GetItemImage(entry), // Définir l'image selon le type
                        SizeMode = PictureBoxSizeMode.Zoom
                    };

                    // Label pour le nom
                    Label nameLabel = new Label
                    {
                        Text = GetItemName(entry),
                        Location = new Point(50, 15),
                        AutoSize = true,
                        Font = new Font("Arial", 12),
                        ForeColor = Color.Black
                    };

                    // Label pour l'extension (si c'est un fichier)
                    Label extLabel = new Label
                    {
                        Text = GetItemExtension(entry),
                        Location = new Point(50 + nameLabel.Width, 15),
                        AutoSize = true,
                        Font = new Font("Arial", 12, FontStyle.Bold),
                        ForeColor = Color.DarkBlue,
                        //Left = nameLabel.Left + nameLabel.Width + 5,
                    };

                    // Ajouter les contrôles au panel
                    AddItemToControls(pictureBox, itemPanel);
                    AddItemToControls(nameLabel, itemPanel);
                    if (!string.IsNullOrEmpty(extLabel.Text))
                        AddItemToControls(extLabel, itemPanel);// Ajouter seulement si c'est un fichier

                    // Rendre le panel cliquable
                    itemPanel.Click += (sender, e) => OnItemClick(sender, e, entry);

                    // Ajouter au panel principal
                    AddItemToControls(itemPanel, mainPanel);

                    yPosition += 60; // Espacement vertical
                }
            }
        }

        /// <summary>
        /// ajoute un controller a un panel sans crash
        /// </summary>
        /// <param name="control">le controller a ajouter</param>
        /// <param name="panel">le panel a qui l'ajouter</param>
        private void AddItemToControls(Control control, Panel panel)
        {
            if (panel.InvokeRequired)
            {
                panel.Invoke(new Action(() =>
                {
                    panel.Controls.Add(control);
                }));
            }
            else
            {
                panel.Controls.Add(control);
            }
        }

        /// <summary>
        /// defini l'image utiliser
        /// </summary>
        /// <param name="entry">l'entre dont depand l'image</param>
        /// <returns>l'image a afficher</returns>
        private Image GetItemImage(string entry)
        {
            if (entry.StartsWith("[D]"))
                return Properties.Resources.dir; // Image dossier
            else
                return Properties.Resources.file; // Image fichier
        }

        /// <summary>
        /// garde le nom du fichier
        /// </summary>
        /// <param name="entry">le fichier</param>
        /// <returns>le nom du fichier</returns>
        private string GetItemName(string entry)
        {
            int startIndex = entry.IndexOf(']') + 2;//+2 car "] "
            //-startIndex car s'est la longeur
            return entry.Contains(".") ? entry.Substring(startIndex, entry.LastIndexOf('.')-startIndex) : entry.Substring(startIndex);
        }

        /// <summary>
        /// obtenir l'extantion d'un fichier
        /// </summary>
        /// <param name="entry">le fichier</param>
        /// <returns>l'extention</returns>
        private string GetItemExtension(string entry)
        {
            if (entry.StartsWith("[F]") && entry.Contains("."))
            {
                return entry.Substring(entry.LastIndexOf('.')); // Extraire uniquement l'extension
            }
            return "";
        }

        /// <summary>
        /// action appeller lors du click sur le flow panel
        /// </summary>
        /// <param name="sender">le flow panel</param>
        /// <param name="e">des infos sur l'event</param>
        /// <param name="entry">l'entre qui a permit de creez le panel</param>
        private void OnItemClick(object sender, EventArgs e, string entry)
        {
            if (entry.StartsWith("[D]"))
            {
                currentPath += entry.Substring(entry.IndexOf(']') + 2) + "\\";
                main.GetRemoteFile(currentPath);
            }
        }

        private void InitializeComponent()
        {
            pathLbl = new Label();
            fileLbl = new Label();
            panelFiles = new Panel();
            panelFiles.SuspendLayout();
            SuspendLayout();
            // 
            // pathLbl
            // 
            pathLbl.Dock = DockStyle.Top;
            pathLbl.Location = new Point(0, 0);
            pathLbl.Name = "pathLbl";
            pathLbl.Size = new Size(624, 39);
            pathLbl.TabIndex = 0;
            pathLbl.Text = "path: ";
            // 
            // fileLbl
            // 
            fileLbl.AutoSize = true;
            fileLbl.BorderStyle = BorderStyle.FixedSingle;
            fileLbl.Location = new Point(0, 0);
            fileLbl.MaximumSize = new Size(624, 0);
            fileLbl.Name = "fileLbl";
            fileLbl.Size = new Size(58, 34);
            fileLbl.TabIndex = 1;
            fileLbl.Text = "File:";
            // 
            // panelFiles
            // 
            panelFiles.AutoScroll = true;
            panelFiles.Controls.Add(fileLbl);
            panelFiles.Dock = DockStyle.Fill;
            panelFiles.Location = new Point(0, 39);
            panelFiles.Name = "panelFiles";
            panelFiles.Size = new Size(624, 320);
            panelFiles.TabIndex = 2;
            // 
            // File
            // 
            ClientSize = new Size(624, 359);
            Controls.Add(panelFiles);
            Controls.Add(pathLbl);
            Name = "File";
            Text = "File";
            panelFiles.ResumeLayout(false);
            panelFiles.PerformLayout();
            ResumeLayout(false);
        }
    }
}
