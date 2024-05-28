﻿using System;
using System.Collections.Generic;
using System.Linq;
using System.Net;
using System.Diagnostics;
using System.Reflection.Metadata.Ecma335;
using System.Text;
using System.Threading.Tasks;
using System.Net.NetworkInformation;

namespace _5TTI_NicolasPonchaut_prosFonc
{
    public struct Fonction
    {
        #region global

        /// <summary>
        /// lire un int sur un consolRead sans crash
        /// </summary>
        /// <param name="question">la question a possez</param>
        /// <returns>la valeur int entrer</returns>
        public int lireInt(string question)
        {
            int n;
            do
            {
                Console.WriteLine(question);
            } while (!int.TryParse(Console.ReadLine(), out n));
            return n;
        }
        /// <summary>
        /// lire un int sur un consolRead sans crash
        /// </summary>
        /// <param name="question">la question a possez</param>
        /// <param name="min">la valeur min</param>
        /// <param name="max">la valeur max</param>
        /// <returns>la valeur int entrer</returns>
        public int lireIntMinMax(string question, int min, int max)
        {
            int n;
            do
            {
                do
                {
                    Console.WriteLine(question);
                } while (!int.TryParse(Console.ReadLine(), out n));
            } while (min <= n && max >= n);
            return n;
        }
        /// <summary>
        /// passer d'un tableau a un string
        /// </summary>
        /// <param name="tab">tableau a transformer</param>
        /// <returns>message avec le tableau en string</returns>
        public string showTabInt(int[] tab)
        {
            string mess = "";
            for (int i = 0; i < tab.Length; i++)
            {
                mess += tab[i];
                mess += "; ";
            }
            return mess;
        }
        /// <summary>
        /// generer un nombre aleatoire
        /// </summary>
        /// <param name="min">nombre min</param>
        /// <param name="max">nombre max</param>
        /// <returns>un nombre aleatoire</returns>
        public int aleNumber(int min, int max)
        {
            Random random = new Random();
            return random.Next(min, max + 1);
        }
        /// <summary>
        /// lancer des de
        /// </summary>
        /// <param name="nbDice">nombre de des</param>
        /// <returns>tableau rempli avec des de lancer</returns>
        public int[] launchDiceTab(int nbDice)
        {
            int[] Tdice = new int[nbDice];
            for (int i = 0; i < nbDice; i++)
            {
                Tdice[i] = aleNumber(1, 6);
            }
            return Tdice;
        }
        /// <summary>
        /// effacer la derniere ligne de la console
        /// </summary>
        public void ClearLastLine()
        {
            int currentLineCursor = Console.CursorTop;
            Console.SetCursorPosition(0, Console.CursorTop - 1);
            Console.Write(new string(' ', Console.WindowWidth));
            Console.SetCursorPosition(0, currentLineCursor - 1);
        }
        /// <summary>
        /// attendre pendant une duree
        /// </summary>
        /// <param name="nombre">le nombre de seconde</param>
        /// <param name="affichage">si on affiche un texte dans la console</param>
        public void sleep(int nombre, bool affichage)
        {
            if (affichage)
            {
                for (int j = 1; j <= nombre; j++)
                {
                    // Add a delay (in this case, 60 seconds)
                    Console.WriteLine($"Vous avez attendu {j - 1} seconde plus que {nombre - j} seconde");
                    Thread.Sleep(1000);
                    // Efface seulement la dernière ligne
                    ClearLastLine();
                }
            }
            else
            {
                for (int j = 1; j <= nombre; j++)
                {
                    Thread.Sleep(1000);
                }
            }
        }
        /// <summary>
        /// telecharger un fichier
        /// </summary>
        /// <param name="url">lurl du fichier</param>
        /// <param name="filePath">l'endroit ou le telecharger</param>
        /// <param name="client">le client qui le telecharge</param>
        /// <param name="affichage">si on fait un affichage console ou pas</param>
        public void downoadFile(string url, string filePath, WebClient client, bool affichage)
        {
            if (affichage)
            {
                Console.ForegroundColor = ConsoleColor.Cyan;
                Console.WriteLine($"Telechargement en cour");
                Console.ResetColor();
                client.DownloadFile(url, filePath);
                ClearLastLine();
            }
            else
            {
                client.DownloadFile(url, filePath);
            }
        }
        /// <summary>
        /// ecrire un texte a la suite du fichier
        /// </summary>
        /// <param name="text">le texte</param>
        /// <param name="filePath">le lien du fichier</param>
        public void writeinFile(string text, string filePath)
        {
            try
            {
                StreamWriter sw = new StreamWriter(filePath, true);
                sw.WriteLine(text);
                sw.Close();
            }
            catch (Exception e)
            {
                Console.WriteLine("Exception: " + e.Message);
            }
        }
        /// <summary>
        /// trier un tableau par ordre croissant avec la methode shell 
        /// </summary>
        /// <param name="tab">tableau rempli avec des int</param>
        /// <returns>un tableau tri par ordre croissant</returns>
        public int[] shellTab(int[] tab)
        {
            int ecart = tab.Length;
            bool swp;
            int swap;
            do
            {
                ecart /= 2;
                do
                {
                    swp = false;
                    for (int i = 0; i < tab.Length - ecart; i++)
                    {
                        if (tab[i] > tab[i + ecart])
                        {
                            swp = true;
                            swap = tab[i];
                            tab[i] = tab[i + ecart];
                            tab[i + ecart] = swap;
                        }
                    }
                } while (swp);
            } while (ecart != 1);
            return tab;
        }

        #endregion

        #region matrice

        /// <summary>
        /// transformer une matrice en string
        /// </summary>
        /// <param name="matrice">matrice non null</param>
        /// <returns>la matrice en string</returns>
        public string matriceIntConcact(int[,] matrice)
        {
            string message = "";
            for (int i = 0; i < matrice.GetLength(0); i++)
            {
                for (int j = 0; j < matrice.GetLength(1); j++)
                {
                    message += matrice[i, j].ToString() + "; ";
                }
                message += "\n";
            }
            return message;
        }
        /// <summary>
        /// transformer une matrice en string
        /// </summary>
        /// <param name="matrice">matrice non null</param>
        /// <returns>la matrice en string</returns>
        public string matriceCharConcact(char[,] matrice)
        {
            string message = "";
            for (int i = 0; i < matrice.GetLength(0); i++)
            {
                for (int j = 0; j < matrice.GetLength(1); j++)
                {
                    message += matrice[i, j].ToString() + "; ";
                }
                message += "\n";
            }
            return message;
        }
        /// <summary>
        /// creez une matrice rempli de chiffre aleatoire
        /// </summary>
        /// <param name="width">largeur</param>
        /// <param name="height">hauteur</param>
        /// <param name="min">val min</param>
        /// <param name="max">val max</param>
        /// <returns>matrice</returns>
        public int[,] matriceIntCreate(int width, int height, int min, int max)
        {
            int[,] matrice = new int[width, height];
            for (int i = 0; i < matrice.GetLength(0); i++)
            {
                for (int j = 0; j < matrice.GetLength(1); j++)
                {
                    matrice[i, j] = aleNumber(min, max);
                }
            }
            return matrice;
        }
        /// <summary>
        /// trouver le determinant d'une matrice
        /// </summary>
        /// <param name="matrice">matrice de int min 2*2 carré</param>
        /// <returns>determinant</returns>
        public int matriceDeterminant(int[,] matrice)
        {
            int diaValPlus = 1;
            int diaValMoin = 1;
            int ValPlus = 0;
            int ValMoin = 0;
            if (matrice.GetLength(0) == 2)
            {
                for (int j = 0; j < matrice.GetLength(1); j++)
                {
                    diaValPlus *= matrice[j, j];
                    diaValMoin *= matrice[j, matrice.GetLength(1) - 1 - j];
                }
                ValMoin += diaValMoin;
                ValPlus += diaValPlus;
            }
            else if (matrice.GetLength(0) > 2)
            {
                for (int i = 0; i < matrice.GetLength(0); i++)
                {
                    for (int j = 0; j < matrice.GetLength(1); j++)
                    {
                        if (i + j < matrice.GetLength(0))
                        {
                            diaValPlus *= matrice[i + j, j];
                            diaValMoin *= matrice[i + j, matrice.GetLength(1) - 1 - j];
                        }
                        else
                        {
                            diaValPlus *= matrice[i + j - matrice.GetLength(0), j];
                            diaValMoin *= matrice[i + j - matrice.GetLength(0), matrice.GetLength(1) - 1 - j];
                        }
                    }
                    ValMoin += diaValMoin;
                    ValPlus += diaValPlus;
                    diaValPlus = 1;
                    diaValMoin = 1;
                }
            }
            return (ValPlus - ValMoin);
        }

        #endregion

        #region binaire

        /// <summary>
        /// transformer un nombre en binaire
        /// </summary>
        /// <param name="number">nombre a transfo</param>
        /// <returns>tableau avec toute les valeur en binaire</returns>
        public int[] binary(int number)
        {
            int[] binary = new int[31];
            int i = 0;
            while (number > 0)
            {
                if (number % 2 == 0)
                {
                    binary[i] = 0;
                }
                else
                {
                    binary[i] = 1;
                }
                number /= 2;
                i++;
            }
            binary = reverseTabInt(binary);
            return binary;
        }
        /// <summary>
        /// afficher du binaire depuis un tableau
        /// </summary>
        /// <param name="binary">tableau de dinaire</param>
        /// <returns>message contenant le binaire</returns>
        public string showBinary(int[] binary)
        {
            string mess = "";
            bool start = binary[0] == 1;
            for (int i = 0; i < binary.Length; i++)
            {
                if (start)
                {
                    mess += binary[i];
                }
                else
                {
                    start = binary[i + 1] == 1;
                }
            }
            return mess;
        }
        /// <summary>
        /// renverser un table de int
        /// </summary>
        /// <param name="tab">table a renverser</param>
        /// <returns>table renverser</returns>
        public int[] reverseTabInt(int[] tab)
        {
            for (int i = 0; i < (tab.Count()) / 2; i++)
            {
                int save = tab[i];
                tab[i] = tab[tab.Count() - i - 1];
                tab[tab.Count() - i - 1] = save;
            }
            return tab;
        }
        /// <summary>
        /// passez de binaire a int
        /// </summary>
        /// <param name="binary">tableau de binaire</param>
        /// <returns>le nombre en code 10</returns>
        public int binToInt(int[] binary)
        {
            int result = 0;
            bool start = binary[0] == 1;
            int z = 0;
            for (int i = 0; i < binary.Count(); i++)
            {
                int num = binary[i];
                if (start)
                {
                    for (int j = 0; j < binary.Count() - 1 - z - i; j++)
                    {
                        num *= 2;
                    }
                    result += num;
                }
                else
                {
                    start = binary[i] == 1;
                    z++;

                }
            }
            return result;
        }
        /// <summary>
        /// passez de int a une table de int
        /// </summary>
        /// <param name="binary">le nombre</param>
        /// <returns>table de int</returns>
        public int[] intToTab(int binary)
        {

            string numberString = binary.ToString();

            int[] result = new int[numberString.Length];

            for (int i = 0; i < numberString.Length; i++)
            {
                result[i] = int.Parse(numberString[i].ToString());
            }
            return result;
        }

        #endregion

        #region demineur


        public void deminReadClassement(string filePath)
        {
            int i = 0;
            List<string> classement = new List<string> { };
            try
            {
                //lecture
                StreamReader sr = new StreamReader(filePath);
                string line = sr.ReadLine();
                //Continue to read until you reach end of file
                while (line != null)
                {
                    classement.Add(line);
                    //Read the next line
                    line = sr.ReadLine();
                    i++;
                }
                //close the file
                sr.Close();

                int[,] tri = new int[classement.Count(), 2];
                //creation matrice tri
                for (int j = 0; j < classement.Count(); j++)
                {
                    string[] subs = classement[j].Split(':');
                    tri[j, 0] = j;
                    int time = 0;
                    for (int s = 1; s < 4; s++) {
                        int.TryParse(subs[s], out int addTime);
                        switch (s)
                        {
                            case 1:
                                time += addTime * 24 * 60;
                                break;
                            case 2:
                                time += addTime * 60;
                                break;
                            case 3:
                                time += addTime;
                                break;
                        }
                    }
                    tri[j, 1] = time;
                }

                //tri par ordre
                int ecart = tri.GetLength(0);
                bool swp;
                int swap;
                do
                {
                    ecart /= 2;
                    do
                    {
                        swp = false;
                        for (int q = 0; q < tri.GetLength(0) - ecart; q++)
                        {
                            if (tri[q, 1] > tri[q + ecart, 1])
                            {
                                swp = true;
                                swap = tri[q, 1];
                                tri[q, 1] = tri[q + ecart, 1];
                                tri[q + ecart, 1] = swap;

                                swap = tri[q, 0];
                                tri[q, 0] = tri[q + ecart, 0];
                                tri[q + ecart, 0] = swap;
                            }
                        }
                    } while (swp);
                } while (ecart != 1);


                string message = "";
                for (int p = 0; p < 10 && p < tri.GetLength(0); p++)
                {
                    message += classement[tri[p, 0]] + "; ";

                    message += "\n";
                }
                Console.WriteLine(message);
            }
            catch (Exception e)
            {
                Console.WriteLine("Exception: " + e.Message);
            }
        }
        /// <summary>
        /// transformer une matrice en string
        /// </summary>
        /// <param name="matrice">matrice non null</param>
        /// <returns>la matrice en string</returns>
        public void deminConcact(string[,] matrice, int x, int y)
        {
            int j = 1;
            if (matrice.GetLength(0) >= 10)
            {
                for (int i = 1; i < matrice.GetLength(0) - 1; i++)
                {
                    if (1 == y && i == x)
                    {
                        Console.BackgroundColor = ConsoleColor.Red;
                    }
                    Console.Write("| ");
                    Console.ResetColor();
                    for (j = 1; j < matrice.GetLength(1) - 1; j++)
                    {
                        if (j == y && i == x)
                        {
                            Console.BackgroundColor = ConsoleColor.Red;
                        }
                        if (i >= 10)
                        {
                            if (matrice[i, j] != null)
                            {
                                
                                if (j == 0)
                                {
                                    Console.Write(matrice[i, j]);
                                }
                                else
                                {
                                    Console.Write(matrice[i, j] + " ");
                                }
                            }
                            else
                            {

                                Console.Write("  ");
                                
                            }
                        }
                        else if(i == 0)
                        {
                            if (j >= 10)
                            {
                                Console.Write(matrice[i, j]);
                            }
                            else
                            {
                                Console.Write(matrice[i, j] + " ");
                            }
                        }
                        else
                        {
                            if (matrice[i, j] != null)
                            {
                                Console.Write(matrice[i, j] + " ");
                            }
                            else
                            {
                                Console.Write("  ");
                            }
                        }
                        if (j + 1 == y && i == x)
                        {
                            Console.BackgroundColor = ConsoleColor.Red;
                        }
                        Console.Write(" | ");
                        Console.ResetColor();
                    }
                    Console.Write("\n");
                }
            }
            else
            {
                for (int i = 1; i < matrice.GetLength(0) - 1; i++)
                {
                    if (1 == y && i == x)
                    {
                        Console.BackgroundColor = ConsoleColor.Red;
                    }
                    Console.Write("| ");
                    Console.ResetColor();
                    for (j = 1; j < matrice.GetLength(1) - 1; j++)
                    {
                        if (j == y && i == x)
                        {
                            Console.BackgroundColor = ConsoleColor.Red;
                        }
                        if (matrice[i, j] != null)
                        {
                            Console.Write(matrice[i, j]);
                        }
                        else
                        {
                            Console.Write(" ");
                        }
                        if (j + 1 == y && i == x)
                        {
                            Console.BackgroundColor = ConsoleColor.Red;
                        }
                        Console.Write(" | ");
                        Console.ResetColor();
                    }
                    Console.Write("\n");
                }
            }
        }
        /// <summary>
        /// placer les bombe dans une matrice
        /// </summary>
        /// <param name="bombe">nombre de bombe</param>
        /// <param name="matrice">la matrice a remplir</param>
        /// <returns></returns>
        public string[,] placementBombe(int bombe, string[,] matrice)
        {
            int i = 0;
            while (i < bombe)
            {
                int pW = aleNumber(1, matrice.GetLength(0) - 2);
                int pH = aleNumber(1, matrice.GetLength(1) - 2);
                if (matrice[pW, pH] != "@")
                {
                    matrice[pW, pH] = "@";
                    i++;
                }
            }
            return matrice;
        }
        /// <summary>
        /// compter les chiffre du demineur
        /// </summary>
        /// <param name="matrice">la matrice ou mettre les chiffre</param>
        /// <returns></returns>
        public string[,] calcNumber(string[,] matrice)
        {
            for (int i = 1; i < matrice.GetLength(0) - 1; i++)
            {
                for (int j = 1; j < matrice.GetLength(1) - 1; j++)
                {
                    if (matrice[i, j] != "@")
                    {
                        int n = 0;
                        for (int y = j - 1; y <= j + 1; y++)
                        {
                            for (int x = i - 1; x <= i + 1; x++)
                            {
                                if (matrice[x, y] == "@")
                                {
                                    n++;
                                }
                            }
                        }
                        matrice[i, j] = n.ToString();
                    }
                }
            }
            return matrice;
        }
        /// <summary>
        /// verifier la case avec la quel il veut interagir
        /// </summary>
        /// <param name="interact">case</param>
        /// <param name="plateau">le plateau afficher</param>
        /// <param name="matrice">le plateau complet</param>
        /// <returns>si il peut interagir ou pas</returns>
        public bool verifCase(string interact, string[,] plateau, string[,] matrice, out int x, out int y)
        {
            x = 0;
            y = 0;
            string[] inter = interact.Split(" ");
            if (inter.Length == 2)
            {
                if (int.TryParse(inter[0], out int w) && int.TryParse(inter[1], out int h))
                {
                    if (0 < w && w < matrice.GetLength(0) && 0 < h && h < matrice.GetLength(1))
                    {
                        if (plateau[w, h] == null || (int)Char.Parse(plateau[w, h]) == 20)
                        {
                            x = w;
                            y = h;
                            return true;
                        }
                        else
                        {
                            return false;
                        }
                    }
                    else
                    {
                        return false;
                    }
                }
                else
                {
                    return false;
                }
            }
            else
            {
                return false;
            }
        }
        /// <summary>
        /// creuser une case
        /// </summary>
        /// <param name="w">position en largeur</param>
        /// <param name="h">position en hauteur</param>
        /// <param name="plateau">le plateau afficher</param>
        /// <param name="matrice">le plateau complet</param>
        /// <returns>si il vie encore ou non</returns>
        public bool creuser(int w, int h, string[,] plateau, string[,] matrice)
        {
            plateau[w, h] = matrice[w, h];
            if (plateau[w, h] == "@")
            {
                return false;
            }
            else
            {
                return true;
            }
        }
        /// <summary>
        /// posez ou retirer un drapeau
        /// </summary>
        /// <param name="w">position en largeur</param>
        /// <param name="h">position en hauteur</param>
        /// <param name="plateau">le plateau afficher</param>
        /// <returns>le plateau de jeux</returns>
        public string[,] drapeau(int w, int h, string[,] plateau, int bombe, ref int nb)
        {
            if (plateau[w, h] == null && bombe > nb)
            {
                plateau[w, h] = ((char)20).ToString();
                nb++;
            }
            else if (plateau[w, h] == ((char)20).ToString())
            {
                plateau[w, h] = null;
                nb--;
            }
            return plateau;
        }
        /// <summary>
        /// creuser une case et si 0 toute les case autour
        /// </summary>
        /// <param name="w">position en largeur</param>
        /// <param name="h">position en hauteur</param>
        /// <param name="plateau">le plateau afficher</param>
        /// <param name="matrice">le plateau complet</param>
        /// <returns>si il meurt ou non</returns>
        public bool creuserAuto(int w, int h, string[,] plateau, string[,] matrice, ref List<string> list)
        {
            plateau[w, h] = matrice[w, h];
            if (plateau[w, h] == "@")
            {
                return false;
            }
            else
            {
                if (plateau[w, h] == "0")
                {
                    for (int y = h - 1; y <= h + 1; y++)
                    {
                        for (int x = w - 1; x <= w + 1; x++)
                        {
                            if (plateau[x, y] == null && !list.Contains($"{x} {y}"))
                            {
                                list.Add($"{x} {y}");
                            }
                        }
                    }
                }
                return true;
            }
        }

        #endregion

        #region cryptage
        public string retireEspaces(string chaine)
        {
            string chaineSSEsp = "";
            int lg = chaine.Length;
            for(int i = 0; i < lg; i++)
            {
                if (chaine[i] != ' ')
                {
                    chaineSSEsp += chaine[i];
                }
            }
            return chaineSSEsp;
        }

        public void creeMat(string cle, string texte, out char[,] matrice)
        {
            int d1 = (texte.Length / cle.Length) + 2;
            int d2 = cle.Length;
            if(texte.Length % cle.Length != 0)
            {
                d1++;
            }
            matrice = new char[d1, d2];
        }

        public void ecritChainesDansMat(string cle, string texte, ref char[,] mat)
        {
            for(int j = 0; j < mat.GetLength(1); j++)
            {
                mat[0, j] = cle[j];
            }
            int k = 0;
            for(int i = 2; i < mat.GetLength(0); i++)
            {
                int j = 0;
                while((j < mat.GetLength(1)) && (k < texte.Length))
                {
                    mat[i, j] = texte[k];
                    k++;
                    j++;
                }
            }
        }

        public void triLigne1(ref char[,] mat)
        {
            bool permut;
            do
            {
                permut = false;
                for(int i = 0; i < mat.GetLength(1) - 1; i++)
                {
                    if (mat[0,i] > mat[0, i + 1])
                    {
                        char x = mat[0, i];
                        mat[0, i] = mat[0, i + 1];
                        mat[0, i + 1] = x;
                        permut = true;
                    }
                }
            } while (!permut);
        }

        public void creeMatriceOutil(string cle, out char[,] matriceTriee)
        {
            matriceTriee = new char[3, cle.Length];
            for(int j = 0; j < matriceTriee.GetLength(1); j++)
            {
                matriceTriee[0,j] = cle[j];
                matriceTriee[2, j] = '0';
            }
            triLigne1(ref matriceTriee);
            for(int j = 1;j <= matriceTriee.GetLength(1); j++)
            {
                matriceTriee[1, j - 1] = Char.Parse(j.ToString());
            }
        }

        public void reporteOrdre(ref char[,] mat, ref char[,] matriceOutil)
        {
            for(int i = 0; i < mat.GetLength(1); i++)
            {
                bool trouve = false;
                int j = 0;
                while(!trouve && j < matriceOutil.GetLength(1))
                {
                    if (mat[0,i] == matriceOutil[0,j] && matriceOutil[2,j] != '1')
                    {
                        mat[1, i] = matriceOutil[1, j];
                        matriceOutil[2, j] = '1';
                        trouve = true;
                    }
                    j++;
                }
            }
        }

        public string construitCryptage(char[,] mat)
        {
            int i = 1;
            string chaineCrypt = "";
            while(i <= mat.GetLength(1))
            {
                bool trouve = false;
                int j = 0;
                while(!trouve && j < mat.GetLength(1))
                {
                    if(Char.Parse(i.ToString()) == mat[1, j])
                    {
                        for(int k = 2; k < mat.GetLength(0); k++)
                        {
                            chaineCrypt += mat[k, j];
                        }
                        chaineCrypt += " ";
                        trouve = true;
                        i++;
                    }
                    
                    j++;
                }
            }
            return chaineCrypt;
        }
        #endregion
    }
}