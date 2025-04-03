using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace RATServeur
{
    public class Cmd : Form
    {
        private Home main;

        private Panel panel;
        private TextBox input;
        private Label terminal;

        public string TerminalText { get { return terminal.Text; } set { terminal.Text = value; } }


        public Cmd(Home mainForm)
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
            panel = new Panel();
            terminal = new Label();
            input = new TextBox();
            panel.SuspendLayout();
            SuspendLayout();
            // 
            // panel
            // 
            panel.AutoScroll = true;
            panel.BackColor = SystemColors.ActiveCaptionText;
            panel.Controls.Add(terminal);
            panel.Dock = DockStyle.Fill;
            panel.ForeColor = SystemColors.Control;
            panel.Location = new Point(0, 0);
            panel.Name = "panel";
            panel.Size = new Size(800, 450);
            panel.TabIndex = 0;
            // 
            // terminal
            // 
            terminal.AutoSize = true;
            terminal.Location = new Point(0, 0);
            terminal.Name = "terminal";
            terminal.Size = new Size(94, 25);
            terminal.TabIndex = 0;
            terminal.Text = "terminal:\n";
            // 
            // input
            // 
            input.Dock = DockStyle.Bottom;
            input.Location = new Point(0, 419);
            input.Name = "input";
            input.Size = new Size(800, 31);
            input.TabIndex = 1;
            // 
            // Cmd
            // 
            AutoScaleDimensions = new SizeF(12F, 25F);
            AutoScaleMode = AutoScaleMode.Font;
            ClientSize = new Size(800, 450);
            Controls.Add(panel);
            Controls.Add(input);
            Name = "Cmd";
            Text = "cmd";
            panel.ResumeLayout(false);
            panel.PerformLayout();
            ResumeLayout(false);

        }

        #endregion

    }
}
