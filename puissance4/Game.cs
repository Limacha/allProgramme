using System;
using System.Collections.Generic;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace puissance4
{
    public class Game : Form
    {
        private TableLayoutPanel grille;
        private Label info;
        private bool redTurn = false;
        private short[,] plateau;

        public Game()
        {
            InitializeComponent();
            Init();
        }

        /// <summary>
        /// initialise le plateau
        /// </summary>
        private void Init()
        {
            //this.Resize += Game_Resize;
            this.FormClosing += Game_FormClosing;
            plateau = new short[7, 6];
            for (int i = 0; i < grille.ColumnCount; i++)
            {
                for (int j = 0; j < grille.RowCount; j++)
                {
                    PictureBox pictureBox = new PictureBox();
                    grille.Controls.Add(pictureBox, i, j);
                    pictureBox.Dock = DockStyle.Fill;
                    //pictureBox.Image = ResizeImage(Properties.Resources.circle, pictureBox.Width, pictureBox.Height);
                    pictureBox.Image = Properties.Resources.circle;
                    pictureBox.Click += PictureBox_Click;
                    pictureBox.SizeMode = PictureBoxSizeMode.Zoom;
                    plateau[i, j] = 0;
                }
            }
            //info.Text = $"R{grille.RowCount} C:{grille.ColumnCount}";
        }

        /// <summary>
        /// lors du click sur une image
        /// </summary>
        /// <param name="sender">l'image</param>
        /// <param name="e">des infos sur l'event</param>
        private void PictureBox_Click(object sender, EventArgs e)
        {
            int position = grille.Controls.GetChildIndex((PictureBox)sender);
            int pX = position / grille.RowCount;
            int pY = position - (pX*grille.RowCount);
            PlacePawn(position, pX, pY);
        }

        /// <summary>
        /// place un pion dans le plateau
        /// </summary>
        /// <param name="position">la position de la picture box</param>
        /// <param name="pX">la position en x</param>
        /// <param name="pY">la position en y</param>
        private void PlacePawn(int position, int pX, int pY)
        {
            int x = pX;
            int y = 0;
            bool block = false;
            while (!block)
            {
                if (y < plateau.GetLength(1) - 1)
                {
                    if (plateau[x, y + 1] == 0)
                    {
                        y++;
                        position++;
                    }
                    else
                    {
                        block = true;
                    }
                }
                else
                {
                    block = true;
                }
            }
            position -= pY;
            if (plateau[x, y] == 0)
            {
                //info.Text = $"p{position} x{x} y:{y}";
                PictureBox pictureBox = (PictureBox)grille.Controls[position];
                //pictureBox.Image = (redTurn) ? ResizeImage(Properties.Resources.circleRed, pictureBox.Width, pictureBox.Height) : ResizeImage(Properties.Resources.circleBlue, pictureBox.Width, pictureBox.Height);
                pictureBox.Image = (redTurn) ? Properties.Resources.circleRed : Properties.Resources.circleBlue;
                plateau[x, y] = (redTurn) ? (short)1 : (short)2;
                if (CheckWin(plateau, (short)x, (short)y, (redTurn) ? (short)1 : (short)2))
                {
                    info.Text = (redTurn) ? "Red win" : "Blue win";
                    MessageBox.Show(info.Text, "Game End", MessageBoxButtons.OK, MessageBoxIcon.Information);

                    // Fermer la fenêtre actuelle (Game)
                    this.Hide();
                }
                else
                {
                    ChangeTurn();
                }
            }
        }

        /// <summary>
        /// change le tour
        /// </summary>
        private void ChangeTurn()
        {
            redTurn = !redTurn;
            info.Text = (redTurn) ? "Turn:Red" : "Turn:Blue";
        }

        /// <summary>
        /// verifie si le joueur a gagner
        /// </summary>
        /// <param name="grid">le plateau</param>
        /// <param name="x">la position en x</param>
        /// <param name="y">la position en y</param>
        /// <param name="player">le joueur qui joue</param>
        /// <returns>si il a gagner</returns>
        private static bool CheckWin(short[,] grid, short x, short y, short player)
        {
            return CheckDirection(grid, x, y, player, 1, 0) ||  // Horizontal |
                   CheckDirection(grid, x, y, player, 0, 1) ||  // Vertical   -
                   CheckDirection(grid, x, y, player, 1, 1) ||  // Diagonale  /
                   CheckDirection(grid, x, y, player, 1, -1);   // Diagonale  \
        }

        /// <summary>
        /// verifie selon un deplacement fournit et son inverse
        /// </summary>
        /// <param name="grid">le plateau</param>
        /// <param name="x">la position en x</param>
        /// <param name="y">la position en y</param>
        /// <param name="player">le joueur qui joue</param>
        /// <param name="dX">le deplacement en x</param>
        /// <param name="dY">le deplacement en y</param>
        /// <returns>verifie la direction</returns>
        private static bool CheckDirection(short[,] grid, short x, short y, short player, int dX, int dY)
        {
            short count = 1; //le pion posser

            // Vérifier dans une direction (ex: droite, haut-droite, haut, etc.)
            count += CountInDirection(grid, x, y, player, dX, dY);

            // Vérifier dans la direction opposée (ex: gauche, bas-gauche, bas, etc.)
            count += CountInDirection(grid, x, y, player, -dX, -dY);

            return count >= 4;
        }

        /// <summary>
        /// count le nombre de pion avec le deplacement donner
        /// </summary>
        /// <param name="grid">le plateau</param>
        /// <param name="x">la position en x</param>
        /// <param name="y">la position en y</param>
        /// <param name="player">le joueur qui joue</param>
        /// <param name="dX">le deplacement en x</param>
        /// <param name="dY">le deplacement en y</param>
        /// <returns>le nombre de pion conter</returns>
        private static short CountInDirection(short[,] grid, short x, short y, short player, int dX, int dY)
        {
            short count = 0;
            int pX = x + dX;
            int pY = y + dY;

            while (pX >= 0 && pX < grid.GetLength(0) && pY >= 0 && pY < grid.GetLength(1) && grid[pX, pY] == player)
            {
                count++;
                pX += dX;
                pY += dY;
            }

            return count;
        }

        /// <summary>
        /// redimensionne l'image pour s'adapter a la bonne taille
        /// </summary>
        /// <param name="img">l'image en question</param>
        /// <param name="width">la largeur de la nouvelle image</param>
        /// <param name="height">la hauteur de la nouvelle image</param>
        /// <returns>la nouvelle image</returns>
        private static Image ResizeImage(Image img, int width, int height)
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
        /// appeler lors de la fermeture de la fentetre
        /// </summary>
        /// <param name="sender">l'object qui a appeller l'event</param>
        /// <param name="e">des infos sur l'event</param>
        private void Game_FormClosing(object sender, FormClosingEventArgs e)
        {

            Home mainForm = Application.OpenForms["Home"] as Home;
            if (mainForm != null)
            {
                mainForm.Show();
            }
            else
            {
                mainForm = new Home();
                mainForm.ShowDialog();
            }
        }

        /// <summary>
        /// appeler lorsque la fenetre change de taille
        /// </summary>
        /// <param name="sender">l'object qui call l'event</param>
        /// <param name="e">des info sur l'event</param>
        private void Game_Resize(object sender, EventArgs e)
        {
            foreach (Control ctrl in grille.Controls)
            {
                if (ctrl is PictureBox pictureBox)
                {
                    //pictureBox.Image = ResizeImage(pictureBox.Image, pictureBox.Width, pictureBox.Height);
                }
            }
        }


        #region Code généré par le Concepteur Windows Form

        /// <summary>
        /// Méthode requise pour la prise en charge du concepteur - ne modifiez pas
        /// le contenu de cette méthode avec l'éditeur de code.
        /// </summary>
        private void InitializeComponent()
        {
            System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(Game));
            this.grille = new System.Windows.Forms.TableLayoutPanel();
            this.info = new System.Windows.Forms.Label();
            this.SuspendLayout();
            // 
            // grille
            // 
            this.grille.ColumnCount = 7;
            this.grille.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 2.38F));
            this.grille.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 2.38F));
            this.grille.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 2.38F));
            this.grille.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 2.38F));
            this.grille.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 2.38F));
            this.grille.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 2.38F));
            this.grille.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 2.38F));
            this.grille.Dock = System.Windows.Forms.DockStyle.Fill;
            this.grille.Location = new System.Drawing.Point(0, 0);
            this.grille.Margin = new System.Windows.Forms.Padding(0);
            this.grille.Name = "grille";
            this.grille.Padding = new System.Windows.Forms.Padding(0, 40, 0, 0);
            this.grille.RowCount = 6;
            this.grille.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 2.38F));
            this.grille.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 2.38F));
            this.grille.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 2.38F));
            this.grille.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 2.38F));
            this.grille.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 2.38F));
            this.grille.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 2.38F));
            this.grille.Size = new System.Drawing.Size(800, 450);
            this.grille.TabIndex = 0;
            // 
            // info
            // 
            this.info.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle;
            this.info.Dock = System.Windows.Forms.DockStyle.Top;
            this.info.Location = new System.Drawing.Point(0, 0);
            this.info.Name = "info";
            this.info.Size = new System.Drawing.Size(800, 30);
            this.info.TabIndex = 0;
            this.info.Text = "Turn:Blue";
            // 
            // Game
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(12F, 25F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(800, 450);
            this.Controls.Add(this.info);
            this.Controls.Add(this.grille);
            this.Icon = ((System.Drawing.Icon)(resources.GetObject("$this.Icon")));
            this.Name = "Game";
            this.Text = "Puissance4";
            this.ResumeLayout(false);

        }

        #endregion
    }
}
