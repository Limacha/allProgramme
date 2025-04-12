using System;
using System.IO;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace puissance4
{
    public partial class Home : Form
    {
        private ListView listGame;
        private OpenFileDialog openFileSave;
        private Button selectFileBtn;
        private Button button1;
        private Label savePathLbl;
        private string selectedFilePath;

        public Home()
        {
            InitializeComponent();
        }


        #region Code généré par le Concepteur Windows Form

        /// <summary>
        /// Méthode requise pour la prise en charge du concepteur - ne modifiez pas
        /// le contenu de cette méthode avec l'éditeur de code.
        /// </summary>
        private void InitializeComponent()
        {
            System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(Home));
            this.button1 = new System.Windows.Forms.Button();
            this.listGame = new System.Windows.Forms.ListView();
            this.openFileSave = new System.Windows.Forms.OpenFileDialog();
            this.selectFileBtn = new System.Windows.Forms.Button();
            this.savePathLbl = new System.Windows.Forms.Label();
            this.SuspendLayout();
            // 
            // button1
            // 
            this.button1.Location = new System.Drawing.Point(12, 363);
            this.button1.Name = "button1";
            this.button1.Size = new System.Drawing.Size(202, 75);
            this.button1.TabIndex = 0;
            this.button1.Text = "Jouer";
            this.button1.UseVisualStyleBackColor = true;
            this.button1.Click += new System.EventHandler(this.button1_Click);
            // 
            // listGame
            // 
            this.listGame.Dock = System.Windows.Forms.DockStyle.Right;
            this.listGame.HideSelection = false;
            this.listGame.Location = new System.Drawing.Point(656, 0);
            this.listGame.Name = "listGame";
            this.listGame.Size = new System.Drawing.Size(144, 450);
            this.listGame.TabIndex = 1;
            this.listGame.UseCompatibleStateImageBehavior = false;
            // 
            // openFileSave
            // 
            this.openFileSave.FileName = "new save";
            this.openFileSave.Filter = "fichier texte | *.txt";
            this.openFileSave.FileOk += new System.ComponentModel.CancelEventHandler(this.openFileDialog1_FileOk);
            // 
            // selectFileBtn
            // 
            this.selectFileBtn.Font = new System.Drawing.Font("Microsoft Sans Serif", 8F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.selectFileBtn.Location = new System.Drawing.Point(12, 320);
            this.selectFileBtn.Name = "selectFileBtn";
            this.selectFileBtn.Size = new System.Drawing.Size(202, 37);
            this.selectFileBtn.TabIndex = 2;
            this.selectFileBtn.Text = "Select save file";
            this.selectFileBtn.UseVisualStyleBackColor = true;
            this.selectFileBtn.Click += new System.EventHandler(this.selectFileBtn_Click);
            // 
            // savePathLbl
            // 
            this.savePathLbl.Dock = System.Windows.Forms.DockStyle.Top;
            this.savePathLbl.Location = new System.Drawing.Point(0, 0);
            this.savePathLbl.Name = "savePathLbl";
            this.savePathLbl.Size = new System.Drawing.Size(656, 23);
            this.savePathLbl.TabIndex = 3;
            this.savePathLbl.Text = "path:";
            // 
            // Home
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(12F, 25F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(800, 450);
            this.Controls.Add(this.savePathLbl);
            this.Controls.Add(this.selectFileBtn);
            this.Controls.Add(this.listGame);
            this.Controls.Add(this.button1);
            this.Icon = ((System.Drawing.Icon)(resources.GetObject("$this.Icon")));
            this.Name = "Home";
            this.Text = "Home";
            this.ResumeLayout(false);

        }

        #endregion

        private void button1_Click(object sender, EventArgs e)
        {
            // Fermer la fenêtre actuelle (Game)
            this.Hide();

            Game game = Application.OpenForms["Game"] as Game;
            if (game != null)
            {
                game.Show();
            }
            else
            {
                game = new Game();
                game.ShowDialog();
            }
        }

        private void openFileDialog1_FileOk(object sender, CancelEventArgs e)
        {
            selectedFilePath = openFileSave.FileName;
            // Vérification de l'existence du fichier
            if (File.Exists(selectedFilePath))
            {
                try
                {
                    // Lire le contenu du fichier
                    string content = File.ReadAllText(selectedFilePath);

                    // Vérification du contenu
                    if (!string.IsNullOrWhiteSpace(content))
                    {
                        if (content.StartsWith("sauvegarde de uno:\n"))
                        {
                            MessageBox.Show("Le fichier est valide et non vide.", "Vérification réussie", MessageBoxButtons.OK, MessageBoxIcon.Information);
                            savePathLbl.Text = "path: " + selectedFilePath;
                        }
                        else
                        {
                            MessageBox.Show("Le fichier est vide.", "Avertissement", MessageBoxButtons.OK, MessageBoxIcon.Warning);
                        }
                    }
                    else
                    {
                        MessageBox.Show("Le fichier est vide.", "Avertissement", MessageBoxButtons.OK, MessageBoxIcon.Warning);
                    }
                }
                catch (Exception ex)
                {
                    MessageBox.Show($"Erreur lors de la lecture du fichier : {ex.Message}", "Erreur", MessageBoxButtons.OK, MessageBoxIcon.Error);
                }
            }
            else
            {
                MessageBox.Show("Le fichier sélectionné n'existe pas.", "Erreur", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
        }

        private void selectFileBtn_Click(object sender, EventArgs e)
        {
            openFileSave.ShowDialog();
        }
    }
}
