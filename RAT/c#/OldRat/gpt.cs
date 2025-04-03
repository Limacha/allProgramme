/*using System;
using System.IO;
using System.Net.Sockets;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

public partial class FormExplorer : Form
{
    private TcpClient client;
    private NetworkStream stream;
    private string currentPath = "C:\\"; // Dossier de départ

    public FormExplorer(TcpClient client)
    {
        InitializeComponent();
        this.client = client;
        this.stream = client.GetStream();
        LoadDirectory(currentPath);
    }

    private void LoadDirectory(string path)
    {
        currentPath = path;
        SendCommand("listdir:" + path);
    }

    private void SendCommand(string command)
    {
        if (client.Connected)
        {
            byte[] data = Encoding.UTF8.GetBytes(command);
            stream.Write(data, 0, data.Length);
            stream.Flush();
        }
    }

    private async void ReceiveResponse()
    {
        byte[] buffer = new byte[8192];
        int bytesRead = await stream.ReadAsync(buffer, 0, buffer.Length);
        string response = Encoding.UTF8.GetString(buffer, 0, bytesRead);
        Invoke(new Action(() => PopulateListView(response)));
    }

    private void PopulateListView(string data)
    {
        listViewFiles.Items.Clear();
        string[] items = data.Split(new[] { "\n" }, StringSplitOptions.RemoveEmptyEntries);

        foreach (string item in items)
        {
            ListViewItem listItem = new ListViewItem(item);
            listViewFiles.Items.Add(listItem);
        }
    }

    private void listViewFiles_ItemActivate(object sender, EventArgs e)
    {
        if (listViewFiles.SelectedItems.Count > 0)
        {
            string selectedItem = listViewFiles.SelectedItems[0].Text;
            if (selectedItem.Contains("[Dossier]"))
            {
                LoadDirectory(Path.Combine(currentPath, selectedItem.Replace("[Dossier] ", "")));
            }
            else
            {
                DownloadFile(Path.Combine(currentPath, selectedItem));
            }
        }
    }

    private void DownloadFile(string filePath)
    {
        SendCommand("download:" + filePath);
    }
}
*/