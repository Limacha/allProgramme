using System;
using System.Collections.Generic;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Runtime.InteropServices.ComTypes;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace OldRat.server
{
    public class File : Form
    {
        private Main main;
        private Label pathLbl;
        private Label fileLbl;
        private Panel panel1;
        private TextBox textBox1;
        private string currentPath = @"C:\"; // Dossier de départ
        public File(Main mainForm)
        {
            main = mainForm;
            InitializeComponent();
            Init();
            main.GetRemoteFile(currentPath);
        }

        private void Init()
        {
        }

        public void ShowFile(string file)
        {
            fileLbl.Text = file;
            textBox1.Text = file;
        }

        private void InitializeComponent()
        {
            this.pathLbl = new System.Windows.Forms.Label();
            this.fileLbl = new System.Windows.Forms.Label();
            this.panel1 = new System.Windows.Forms.Panel();
            this.textBox1 = new System.Windows.Forms.TextBox();
            this.panel1.SuspendLayout();
            this.SuspendLayout();
            // 
            // pathLbl
            // 
            this.pathLbl.Dock = System.Windows.Forms.DockStyle.Top;
            this.pathLbl.Location = new System.Drawing.Point(0, 0);
            this.pathLbl.Name = "pathLbl";
            this.pathLbl.Size = new System.Drawing.Size(624, 25);
            this.pathLbl.TabIndex = 0;
            this.pathLbl.Text = "path: ";
            // 
            // fileLbl
            // 
            this.fileLbl.AutoSize = true;
            this.fileLbl.Location = new System.Drawing.Point(0, 0);
            this.fileLbl.MaximumSize = new System.Drawing.Size(624, 0);
            this.fileLbl.Name = "fileLbl";
            this.fileLbl.Size = new System.Drawing.Size(53, 25);
            this.fileLbl.TabIndex = 1;
            this.fileLbl.Text = "File:";
            // 
            // panel1
            // 
            this.panel1.AutoScroll = true;
            this.panel1.Controls.Add(this.textBox1);
            this.panel1.Controls.Add(this.fileLbl);
            this.panel1.Dock = System.Windows.Forms.DockStyle.Fill;
            this.panel1.Location = new System.Drawing.Point(0, 25);
            this.panel1.Name = "panel1";
            this.panel1.Size = new System.Drawing.Size(624, 334);
            this.panel1.TabIndex = 2;
            // 
            // textBox1
            // 
            this.textBox1.Dock = System.Windows.Forms.DockStyle.Bottom;
            this.textBox1.Location = new System.Drawing.Point(0, 303);
            this.textBox1.Name = "textBox1";
            this.textBox1.Size = new System.Drawing.Size(624, 31);
            this.textBox1.TabIndex = 2;
            // 
            // File
            // 
            this.ClientSize = new System.Drawing.Size(624, 359);
            this.Controls.Add(this.panel1);
            this.Controls.Add(this.pathLbl);
            this.Name = "File";
            this.Text = "File";
            this.panel1.ResumeLayout(false);
            this.panel1.PerformLayout();
            this.ResumeLayout(false);

        }

        private void pictureBox1_Click(object sender, EventArgs e)
        {

        }
    }
}
