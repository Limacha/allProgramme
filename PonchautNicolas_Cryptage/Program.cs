using _5TTI_NicolasPonchaut_prosFonc;
using System.Data;

namespace PonchautNicolas_Cryptage
{
    internal class Program
    {
        static void Main(string[] args)
        {
            Fonction fonction = new Fonction();
            string text;
            string cle;
            do
            {
                Console.WriteLine("Quelle texte voullez vous crypter?");
                text = Console.ReadLine();
                Console.WriteLine("avec quel mot?");
                cle = Console.ReadLine();
            } while (cle.Length > text.Length || cle.Length > 9);

            string noSpace = fonction.retireEspaces(text);
            Console.WriteLine(noSpace);

            fonction.creeMat(cle, noSpace, out char[,] mat);
            
            fonction.ecritChainesDansMat(cle, noSpace, ref mat);
            Console.WriteLine(fonction.matriceCharConcact(mat));
            
            fonction.creeMatriceOutil(cle, out char[,] matTrav);
            Console.WriteLine(fonction.matriceCharConcact(matTrav));
            
            fonction.reporteOrdre(ref mat, ref matTrav);
            Console.WriteLine(fonction.matriceCharConcact(mat) + "\n");
            Console.WriteLine(fonction.matriceCharConcact(matTrav));
            
            string message = fonction.construitCryptage(mat);
            Console.WriteLine(message);
        }
    }
}