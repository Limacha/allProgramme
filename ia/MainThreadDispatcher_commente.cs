// ── Importation des espaces de noms nécessaires ──────────────────────────────

using System;                    // Fournit : Action, Func<T>, Exception — types de base
using System.Collections.Generic; // Fournit : Queue<T> — file d'attente FIFO
using UnityEngine;               // Fournit : MonoBehaviour, GameObject, Debug — base Unity

// ── Déclaration de la classe ──────────────────────────────────────────────────

// 'public'         : accessible depuis tous les scripts
// 'MonoBehaviour'  : hérite de MonoBehaviour pour avoir accès à Update()
//                    Le dispatcher a besoin d'Update() pour vider sa file chaque frame
//                    C'est pourquoi ce n'est PAS une classe static comme EventDispatcher
public class MainThreadDispatcher : MonoBehaviour
{
    // ════════════════════════════════════════════════════════════════════════
    //   SINGLETON — Une seule instance pour tout le jeu
    // ════════════════════════════════════════════════════════════════════════

    // Référence statique à l'unique instance du dispatcher dans la scène
    // 'static' : partagée à travers tout le projet, pas liée à un objet particulier
    private static MainThreadDispatcher _instance;

    // Propriété publique pour accéder à l'instance depuis n'importe où
    // Si l'instance n'existe pas encore, la crée automatiquement (lazy initialization)
    public static MainThreadDispatcher Instance
    {
        get
        {
            if (_instance == null)    // L'instance n'existe pas encore
                CreateInstance();     // On la crée automatiquement
            return _instance;         // Retourne l'instance (fraîchement créée ou déjà existante)
        }
    }


    // ════════════════════════════════════════════════════════════════════════
    //   FILE D'ATTENTE THREAD-SAFE
    // ════════════════════════════════════════════════════════════════════════

    // Queue<Action> : file FIFO (premier entré = premier exécuté)
    // Stocke les actions envoyées depuis des threads secondaires,
    // en attente d'être exécutées sur le thread principal
    // 'readonly' : la référence à la file ne change jamais (on ajoute/retire des éléments, pas la file elle-même)
    private readonly Queue<Action> _queue = new Queue<Action>();

    // Objet verrou pour la synchronisation entre threads
    // 'object' simple : en C#, n'importe quel objet peut servir de verrou pour 'lock'
    // 'readonly' : le verrou ne doit jamais changer d'objet de référence
    private readonly object _lock = new object();


    // ════════════════════════════════════════════════════════════════════════
    //   INITIALISATION AUTOMATIQUE
    // ════════════════════════════════════════════════════════════════════════

    // [RuntimeInitializeOnLoadMethod] : attribut Unity qui appelle cette méthode
    // automatiquement au démarrage du jeu, AVANT le chargement de la première scène
    // → Aucun setup manuel nécessaire dans l'éditeur ou dans un autre script
    [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.BeforeSceneLoad)]
    private static void CreateInstance()
    {
        if (_instance != null) return; // Sécurité : si l'instance existe déjà, ne rien faire

        var go = new GameObject("[MainThreadDispatcher]"); // Crée un GameObject vide dans la scène
        _instance = go.AddComponent<MainThreadDispatcher>(); // Attache ce script au GameObject
                                                              // → Unity appellera Update() chaque frame
        DontDestroyOnLoad(go); // Empêche Unity de détruire ce GameObject lors des changements de scène
                                // → Le dispatcher persiste pendant toute la durée du jeu
    }


    // ════════════════════════════════════════════════════════════════════════
    //   MÉTHODE : Enqueue — Fire & Forget (sans attendre le résultat)
    // ════════════════════════════════════════════════════════════════════════

    // Enfile une action pour qu'elle soit exécutée sur le thread principal au prochain Update()
    // Peut être appelé depuis N'IMPORTE QUEL thread (secondaire, async, Task.Run...)
    public static void Enqueue(Action action)
    {
        if (action == null) return; // Sécurité : ignorer les actions nulles

        // 'lock(_lock)' : verrouillage exclusif
        // Un seul thread à la fois peut entrer dans ce bloc
        // Si un thread A est en train d'ajouter et qu'un thread B arrive → B attend que A finisse
        // Empêche la corruption de la file (race condition)
        lock (Instance._lock)
            Instance._queue.Enqueue(action); // Ajoute l'action à la fin de la file
    }


    // ════════════════════════════════════════════════════════════════════════
    //   MÉTHODE : EnqueueAsync — Attend que l'action soit exécutée
    // ════════════════════════════════════════════════════════════════════════

    // Version async : retourne une Task que l'appelant peut 'await'
    // Le thread secondaire est suspendu jusqu'à ce que l'action soit exécutée sur le thread principal
    public static System.Threading.Tasks.Task EnqueueAsync(Action action)
    {
        // TaskCompletionSource : permet de contrôler manuellement quand une Task est "complète"
        // Le thread secondaire fera 'await' sur tcs.Task, qui se complétera quand on appelle SetResult
        var tcs = new System.Threading.Tasks.TaskCompletionSource<bool>();

        // On enfile une action qui WRAPE l'action originale + la complétion de la Task
        Enqueue(() =>
        {
            try
            {
                action();          // Exécute l'action sur le thread principal
                tcs.SetResult(true); // Signale au thread secondaire que c'est fait
                                     // → le 'await EnqueueAsync(...)' se débloque
            }
            catch (Exception ex)
            {
                tcs.SetException(ex); // Si une erreur survient, la propage à l'appelant async
                                      // → le 'await' lèvera l'exception chez l'appelant
            }
        });

        return tcs.Task; // Retourne la Task sur laquelle l'appelant peut faire 'await'
    }

    // Surcharge générique avec valeur de retour
    // Func<T> : une fonction qui retourne une valeur de type T
    // Task<T> : la Task qui se complétera avec cette valeur de retour
    public static System.Threading.Tasks.Task<T> EnqueueAsync<T>(Func<T> func)
    {
        var tcs = new System.Threading.Tasks.TaskCompletionSource<T>(); // Task typée avec valeur de retour

        Enqueue(() =>
        {
            try   { tcs.SetResult(func()); }           // Exécute func() et stocke son résultat dans la Task
            catch (Exception ex) { tcs.SetException(ex); } // Propage l'erreur si func() plante
        });

        return tcs.Task; // L'appelant récupère la valeur avec : var result = await EnqueueAsync(() => GetValue());
    }


    // ════════════════════════════════════════════════════════════════════════
    //   MÉTHODE : Update — Cœur du dispatcher (appelé chaque frame par Unity)
    // ════════════════════════════════════════════════════════════════════════

    // Update() est appelé par Unity sur le thread principal à chaque frame
    // C'est ici que les actions en attente sont réellement exécutées
    private void Update()
    {
        const int maxPerFrame = 20; // Limite : max 20 actions exécutées par frame
                                    // Évite les pics de performance si la file est très chargée
                                    // Les actions restantes seront exécutées aux frames suivantes
        int count = 0; // Compteur d'actions exécutées dans cette frame

        while (count < maxPerFrame) // Continue jusqu'à la limite ou file vide
        {
            Action action; // Variable pour stocker l'action à exécuter (déclarée hors du lock)

            lock (_lock) // Verrou pour lire/retirer de la file de façon thread-safe
                         // Nécessaire car un thread secondaire pourrait ajouter en même temps
            {
                if (_queue.Count == 0) break; // File vide → on sort de la boucle while
                action = _queue.Dequeue();    // Retire et récupère la première action de la file
            }
            // Important : l'action est exécutée HORS du lock
            // Pourquoi ? Si on exécutait dans le lock et que l'action appelle Enqueue(),
            // on aurait un deadlock (le thread essaierait de prendre un lock qu'il tient déjà)

            try   { action(); } // Exécute l'action sur le thread principal ✅
            catch (Exception ex) { Debug.LogError($"[MainThreadDispatcher] {ex}"); } // Log l'erreur sans planter le jeu
            count++; // Incrémente le compteur
        }
    }


    // ════════════════════════════════════════════════════════════════════════
    //   MÉTHODE : OnDestroy — Nettoyage lors de la destruction
    // ════════════════════════════════════════════════════════════════════════

    // Appelé par Unity quand ce GameObject est détruit
    private void OnDestroy()
    {
        // Nettoie la référence statique SEULEMENT si c'est bien cette instance qui est détruite
        // Sécurité : évite de nullifier l'instance si une autre a été créée entre-temps
        if (_instance == this) _instance = null;
    }
}


// ════════════════════════════════════════════════════════════════════════════
//   EXEMPLE D'UTILISATION (mis en commentaire)
// ════════════════════════════════════════════════════════════════════════════

/*
public class NetworkManager : MonoBehaviour
{
    private async void Start()
    {
        // Task.Run() : démarre le code dans un thread secondaire (pool de threads .NET)
        // 'await' : suspend Start() ici sans bloquer Unity, reprend quand la Task est finie
        await System.Threading.Tasks.Task.Run(async () =>
        {
            // ⚠️ Ici on est sur un THREAD SECONDAIRE
            // On ne peut PAS toucher à des GameObjects ici → Unity lèverait une exception

            await System.Threading.Tasks.Task.Delay(2000); // Simule 2 secondes de latence réseau
            string result = "Données reçues !";

            // Solution : on confie la modification d'UI au thread principal via Enqueue
            MainThreadDispatcher.Enqueue(() =>
            {
                // ✅ Ici on est de retour sur le THREAD PRINCIPAL (appelé depuis Update())
                Debug.Log(result);          // Sûr
                // myText.text = result;    // Sûr — modification d'un composant Unity
            });
        });
    }
}
*/
