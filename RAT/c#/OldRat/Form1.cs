using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Reflection.Emit;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;
using OldRat.server;

namespace OldRat
{
    public partial class Form1 : Form
    {
        public Form1()
        {
            InitializeComponent();
        }

        private void button1_Click_1(object sender, EventArgs e)
        {
            RATServer server = new RATServer();
            this.Hide(); // close form 1
            server.ShowDialog(); //affiche et oblige a rester sur lui
        }

        private void button2_Click(object sender, EventArgs e)
        {
            RATClient client = new RATClient();
            this.Hide(); // close form 1
            client.ShowDialog(); //affiche et oblige a rester sur lui
        }

        private void button3_Click(object sender, EventArgs e)
        {
            Main main = new Main();
            this.Hide(); // close form 1
            main.Show(); //affiche
        }
    }
}
