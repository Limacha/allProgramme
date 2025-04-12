using Microsoft.Win32;
using System;
using System.Collections.Generic;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Net.Sockets;
using System.Reflection;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace Global
{
    public static class Fonction
    {
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

        /// <summary>
        /// modifier une variable depuis un string
        /// </summary>
        /// <param name="inst">la class qui contient la variable</param>
        /// <param name="varName">le nom de la variable</param>
        /// <param name="newVar">la nouvelle valeur</param>
        /// <returns></returns>
        public static bool EditVar(object inst, string varName, object newVar)
        {
            // Réflexion sur soi-même
            Type type = inst.GetType();
            FieldInfo champ = type.GetField(varName,
                BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Instance);

            if (champ != null)
            {
                champ.SetValue(inst, newVar);
                return true;
            }
            else
            {
                return false;
            }
        }

        /// <summary>
        /// convertit un string en une valeur de type fournit
        /// </summary>
        /// <param name="valeurStr">la valeur a convertir</param>
        /// <param name="typeStr">le type dans le quel convertir defini</param>
        /// <returns>la valeur convertit si possible si echec: valeur</returns>
        public static object ConvertirValeur(string valeurStr, string typeStr)
        {
            try
            {
                TypeCode code = Type.GetTypeCode(Type.GetType(typeStr, false, true));

                switch (code)
                {
                    case (TypeCode.Boolean): return bool.Parse(valeurStr);
                    case TypeCode.Int32: return int.Parse(valeurStr);
                    case TypeCode.Int64: return long.Parse(valeurStr);
                    case TypeCode.Single: return float.Parse(valeurStr);
                    case TypeCode.Double: return double.Parse(valeurStr);
                    case TypeCode.String: return valeurStr;
                    case TypeCode.Char: return valeurStr[0];
                    case TypeCode.Byte: return byte.Parse(valeurStr);
                    case TypeCode.UInt32: return uint.Parse(valeurStr);
                    case TypeCode.UInt64: return ulong.Parse(valeurStr);
                    case TypeCode.Int16: return short.Parse(valeurStr);
                    case TypeCode.UInt16: return ushort.Parse(valeurStr);
                    case TypeCode.Decimal: return decimal.Parse(valeurStr);
                    default: return "echec: " + valeurStr; // fallback si type inconnu
                };
            }
            catch
            {
                return valeurStr; // retour brut si erreur
            }
        }
    }
}
