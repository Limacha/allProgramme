using static Global.Fonction;
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace RATWinFormApp1.server
{
    public class FormStream : Form
    {
        private PictureBox pictureBox;
        private Image image;
        private MenuStrip menuStrip1;
        private ToolStripMenuItem delayToolStripMenuItem;
        private TrackBar delayBar;
        private SplitContainer delayContainer;
        private Label delayLbl;
        private int delay = 250;

        public Image Img { get { return image; } set { image = value; } }

        public FormStream(Image image)
        {
            InitializeComponent();
            Init();
            this.image = image;
        }

        private void Init()
        {
            // Gestion de la fermeture de la fenêtre
            this.FormClosing += new FormClosingEventHandler(OnFormClosing);
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

        private void Always()
        {
            while (true)
            {
                delay = delayBar.Value;
                delayLbl.Text = delay.ToString();
                pictureBox.Image = ResizeImage(image, pictureBox.Width, pictureBox.Height);
                Thread.Sleep(delay);
            }
        }


        /// <summary>
        /// appeler lors de la fermeture de la fenetre
        /// </summary>
        /// <param name="sender">object qui a declencher l'event</param>
        /// <param name="e">information lieu a l'event</param>
        private void OnFormClosing(object sender, FormClosingEventArgs e)
        {
        }


        #region Windows Form Designer generated code

        /// <summary>
        /// Required method for Designer support - do not modify
        /// the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent()
        {
            this.pictureBox = new System.Windows.Forms.PictureBox();
            this.menuStrip1 = new System.Windows.Forms.MenuStrip();
            this.delayToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.delayBar = new System.Windows.Forms.TrackBar();
            this.delayContainer = new System.Windows.Forms.SplitContainer();
            this.delayLbl = new System.Windows.Forms.Label();
            ((System.ComponentModel.ISupportInitialize)(this.pictureBox)).BeginInit();
            this.menuStrip1.SuspendLayout();
            ((System.ComponentModel.ISupportInitialize)(this.delayBar)).BeginInit();
            ((System.ComponentModel.ISupportInitialize)(this.delayContainer)).BeginInit();
            this.delayContainer.Panel1.SuspendLayout();
            this.delayContainer.Panel2.SuspendLayout();
            this.delayContainer.SuspendLayout();
            this.SuspendLayout();
            // 
            // pictureBox
            // 
            this.pictureBox.Dock = System.Windows.Forms.DockStyle.Fill;
            this.pictureBox.Image = global::RATWinFormApp1.Properties.Resources.nothing;
            this.pictureBox.Location = new System.Drawing.Point(0, 40);
            this.pictureBox.Name = "pictureBox";
            this.pictureBox.Size = new System.Drawing.Size(800, 410);
            this.pictureBox.SizeMode = System.Windows.Forms.PictureBoxSizeMode.Zoom;
            this.pictureBox.TabIndex = 0;
            this.pictureBox.TabStop = false;
            // 
            // menuStrip1
            // 
            this.menuStrip1.GripMargin = new System.Windows.Forms.Padding(2, 2, 0, 2);
            this.menuStrip1.ImageScalingSize = new System.Drawing.Size(32, 32);
            this.menuStrip1.Items.AddRange(new System.Windows.Forms.ToolStripItem[] {
            this.delayToolStripMenuItem});
            this.menuStrip1.Location = new System.Drawing.Point(0, 0);
            this.menuStrip1.Name = "menuStrip1";
            this.menuStrip1.Size = new System.Drawing.Size(800, 40);
            this.menuStrip1.TabIndex = 1;
            this.menuStrip1.Text = "menuStrip1";
            // 
            // delayToolStripMenuItem
            // 
            this.delayToolStripMenuItem.Name = "delayToolStripMenuItem";
            this.delayToolStripMenuItem.Size = new System.Drawing.Size(91, 36);
            this.delayToolStripMenuItem.Text = "delay";
            this.delayToolStripMenuItem.Click += new System.EventHandler(this.DelayToolStripMenuItem_Click);
            // 
            // delayBar
            // 
            this.delayBar.AutoSize = false;
            this.delayBar.Dock = System.Windows.Forms.DockStyle.Fill;
            this.delayBar.Location = new System.Drawing.Point(0, 0);
            this.delayBar.Maximum = 1000;
            this.delayBar.Minimum = 50;
            this.delayBar.Name = "delayBar";
            this.delayBar.Size = new System.Drawing.Size(706, 30);
            this.delayBar.TabIndex = 2;
            this.delayBar.Value = 250;
            // 
            // delayContainer
            // 
            this.delayContainer.Dock = System.Windows.Forms.DockStyle.Top;
            this.delayContainer.IsSplitterFixed = true;
            this.delayContainer.Location = new System.Drawing.Point(0, 40);
            this.delayContainer.Name = "delayContainer";
            // 
            // delayContainer.Panel1
            // 
            this.delayContainer.Panel1.Controls.Add(this.delayLbl);
            // 
            // delayContainer.Panel2
            // 
            this.delayContainer.Panel2.Controls.Add(this.delayBar);
            this.delayContainer.Size = new System.Drawing.Size(800, 30);
            this.delayContainer.SplitterDistance = 90;
            this.delayContainer.TabIndex = 3;
            // 
            // delayLbl
            // 
            this.delayLbl.Location = new System.Drawing.Point(0, 0);
            this.delayLbl.Name = "delayLbl";
            this.delayLbl.Size = new System.Drawing.Size(86, 25);
            this.delayLbl.TabIndex = 0;
            this.delayLbl.Text = "250";
            // 
            // FormStream
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(12F, 25F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(800, 450);
            this.Controls.Add(this.delayContainer);
            this.Controls.Add(this.pictureBox);
            this.Controls.Add(this.menuStrip1);
            this.MainMenuStrip = this.menuStrip1;
            this.Name = "FormStream";
            this.Text = "screen";
            ((System.ComponentModel.ISupportInitialize)(this.pictureBox)).EndInit();
            this.menuStrip1.ResumeLayout(false);
            this.menuStrip1.PerformLayout();
            ((System.ComponentModel.ISupportInitialize)(this.delayBar)).EndInit();
            this.delayContainer.Panel1.ResumeLayout(false);
            this.delayContainer.Panel2.ResumeLayout(false);
            ((System.ComponentModel.ISupportInitialize)(this.delayContainer)).EndInit();
            this.delayContainer.ResumeLayout(false);
            this.ResumeLayout(false);
            this.PerformLayout();

        }

        #endregion

        private void DelayToolStripMenuItem_Click(object sender, EventArgs e)
        {
            delayContainer.Enabled = !delayContainer.Enabled;
            delayContainer.Visible = !delayContainer.Visible;
        }
    }
}
