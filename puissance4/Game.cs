using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace puissance4
{
    public class Game : Form
    {
        private TableLayoutPanel grille;
        private Button button1;
        private Button button2;
        private Button button3;
        private Button button4;

        public Game()
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
            this.grille = new System.Windows.Forms.TableLayoutPanel();
            this.button1 = new System.Windows.Forms.Button();
            this.button2 = new System.Windows.Forms.Button();
            this.button3 = new System.Windows.Forms.Button();
            this.button4 = new System.Windows.Forms.Button();
            this.grille.SuspendLayout();
            this.SuspendLayout();
            // 
            // grille
            // 
            this.grille.ColumnCount = 7;
            this.grille.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 100F));
            this.grille.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle());
            this.grille.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle());
            this.grille.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle());
            this.grille.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle());
            this.grille.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle());
            this.grille.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle());
            this.grille.Controls.Add(this.button1, 0, 0);
            this.grille.Controls.Add(this.button2, 1, 0);
            this.grille.Controls.Add(this.button3, 1, 1);
            this.grille.Controls.Add(this.button4, 0, 1);
            this.grille.Dock = System.Windows.Forms.DockStyle.Fill;
            this.grille.Location = new System.Drawing.Point(0, 0);
            this.grille.Name = "grille";
            this.grille.RowCount = 6;
            this.grille.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 20F));
            this.grille.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.grille.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.grille.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.grille.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.grille.RowStyles.Add(new System.Windows.Forms.RowStyle());
            this.grille.Size = new System.Drawing.Size(800, 450);
            this.grille.TabIndex = 0;
            // 
            // button1
            // 
            this.button1.Location = new System.Drawing.Point(3, 3);
            this.button1.Name = "button1";
            this.button1.Size = new System.Drawing.Size(75, 14);
            this.button1.TabIndex = 0;
            this.button1.Text = "button1";
            this.button1.UseVisualStyleBackColor = true;
            // 
            // button2
            // 
            this.button2.Location = new System.Drawing.Point(722, 3);
            this.button2.Name = "button2";
            this.button2.Size = new System.Drawing.Size(75, 14);
            this.button2.TabIndex = 1;
            this.button2.Text = "button2";
            this.button2.UseVisualStyleBackColor = true;
            // 
            // button3
            // 
            this.button3.Location = new System.Drawing.Point(722, 23);
            this.button3.Name = "button3";
            this.button3.Size = new System.Drawing.Size(75, 23);
            this.button3.TabIndex = 2;
            this.button3.Text = "button3";
            this.button3.UseVisualStyleBackColor = true;
            // 
            // button4
            // 
            this.button4.Location = new System.Drawing.Point(3, 23);
            this.button4.Name = "button4";
            this.button4.Size = new System.Drawing.Size(75, 23);
            this.button4.TabIndex = 3;
            this.button4.Text = "button4";
            this.button4.UseVisualStyleBackColor = true;
            // 
            // Game
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(12F, 25F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(800, 450);
            this.Controls.Add(this.grille);
            this.Name = "Game";
            this.Text = "Puissance4";
            this.grille.ResumeLayout(false);
            this.ResumeLayout(false);

        }

        #endregion
    }
}
