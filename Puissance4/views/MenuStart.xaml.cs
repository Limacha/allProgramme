using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;

namespace Puissance4.views
{
    /// <summary>
    /// Logique d'interaction pour Menu.xaml
    /// </summary>
    public partial class MenuStart : Page
    {
        public MenuStart()
        {
            InitializeComponent();
            InitComponent();
        }
        public void InitComponent()
        {
            JouerBtn.Click += new RoutedEventHandler(ChangeToGame);
        }

        private void ChangeToGame(object sender, RoutedEventArgs e)
        {
            MainWindow pagePrincipale = (MainWindow)App.Current.MainWindow;
            pagePrincipale.Contenu.Content = new Game();
        }
    }
}
