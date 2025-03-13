using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace RATWinFormApp1.server
{
    public class File : Form
    {

        public File()
        {
            InitializeComponent();
        }

        private void InitializeComponent()
        {
            this.SuspendLayout();
            // 
            // File
            // 
            this.ClientSize = new System.Drawing.Size(624, 359);
            this.Name = "File";
            this.Text = "File";
            this.ResumeLayout(false);

        }
    }
}
