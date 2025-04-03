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

namespace RATServeur
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
            FormClosing += new FormClosingEventHandler(OnFormClosing);
            Load += BackgroundTask;
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
            pictureBox = new PictureBox();
            menuStrip1 = new MenuStrip();
            delayToolStripMenuItem = new ToolStripMenuItem();
            delayBar = new TrackBar();
            delayContainer = new SplitContainer();
            delayLbl = new Label();
            ((ISupportInitialize)pictureBox).BeginInit();
            menuStrip1.SuspendLayout();
            ((ISupportInitialize)delayBar).BeginInit();
            ((ISupportInitialize)delayContainer).BeginInit();
            delayContainer.Panel1.SuspendLayout();
            delayContainer.Panel2.SuspendLayout();
            delayContainer.SuspendLayout();
            SuspendLayout();
            // 
            // pictureBox
            // 
            pictureBox.Dock = DockStyle.Fill;
            pictureBox.Image = RATServeur.Properties.Resources.nothing;
            pictureBox.Location = new Point(0, 40);
            pictureBox.Name = "pictureBox";
            pictureBox.Size = new Size(800, 410);
            pictureBox.SizeMode = PictureBoxSizeMode.Zoom;
            pictureBox.TabIndex = 0;
            pictureBox.TabStop = false;
            // 
            // menuStrip1
            // 
            menuStrip1.GripMargin = new Padding(2, 2, 0, 2);
            menuStrip1.ImageScalingSize = new Size(32, 32);
            menuStrip1.Items.AddRange(new ToolStripItem[] {
            delayToolStripMenuItem});
            menuStrip1.Location = new Point(0, 0);
            menuStrip1.Name = "menuStrip1";
            menuStrip1.Size = new Size(800, 40);
            menuStrip1.TabIndex = 1;
            menuStrip1.Text = "menuStrip1";
            // 
            // delayToolStripMenuItem
            // 
            delayToolStripMenuItem.Name = "delayToolStripMenuItem";
            delayToolStripMenuItem.Size = new Size(91, 36);
            delayToolStripMenuItem.Text = "delay";
            delayToolStripMenuItem.Click += new EventHandler(DelayToolStripMenuItem_Click);
            // 
            // delayBar
            // 
            delayBar.AutoSize = false;
            delayBar.Dock = DockStyle.Fill;
            delayBar.Location = new Point(0, 0);
            delayBar.Maximum = 1000;
            delayBar.Minimum = 50;
            delayBar.Name = "delayBar";
            delayBar.Size = new Size(706, 30);
            delayBar.TabIndex = 2;
            delayBar.Value = 250;
            // 
            // delayContainer
            // 
            delayContainer.Dock = DockStyle.Top;
            delayContainer.IsSplitterFixed = true;
            delayContainer.Location = new Point(0, 40);
            delayContainer.Name = "delayContainer";
            // 
            // delayContainer.Panel1
            // 
            delayContainer.Panel1.Controls.Add(delayLbl);
            // 
            // delayContainer.Panel2
            // 
            delayContainer.Panel2.Controls.Add(delayBar);
            delayContainer.Size = new Size(800, 30);
            delayContainer.SplitterDistance = 90;
            delayContainer.TabIndex = 3;
            // 
            // delayLbl
            // 
            delayLbl.Location = new Point(0, 0);
            delayLbl.Name = "delayLbl";
            delayLbl.Size = new Size(86, 25);
            delayLbl.TabIndex = 0;
            delayLbl.Text = "250";
            // 
            // FormStream
            // 
            AutoScaleDimensions = new SizeF(12F, 25F);
            AutoScaleMode = AutoScaleMode.Font;
            ClientSize = new Size(800, 450);
            Controls.Add(delayContainer);
            Controls.Add(pictureBox);
            Controls.Add(menuStrip1);
            MainMenuStrip = menuStrip1;
            Name = "FormStream";
            Text = "screen";
            ((ISupportInitialize)pictureBox).EndInit();
            menuStrip1.ResumeLayout(false);
            menuStrip1.PerformLayout();
            ((ISupportInitialize)delayBar).EndInit();
            delayContainer.Panel1.ResumeLayout(false);
            delayContainer.Panel2.ResumeLayout(false);
            ((ISupportInitialize)delayContainer).EndInit();
            delayContainer.ResumeLayout(false);
            ResumeLayout(false);
            PerformLayout();

        }

        #endregion

        private void DelayToolStripMenuItem_Click(object sender, EventArgs e)
        {
            delayContainer.Enabled = !delayContainer.Enabled;
            delayContainer.Visible = !delayContainer.Visible;
        }
    }
}
