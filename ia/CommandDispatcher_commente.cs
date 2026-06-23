// ── Importation des espaces de noms nécessaires ──────────────────────────────

using System;                    // Fournit : Action<T>, Exception
using System.Collections.Generic; // Fournit : Stack<T>, List<T>
using UnityEngine;               // Fournit : MonoBehaviour, Time, Input, GameObject

// ── Déclaration de la classe ──────────────────────────────────────────────────

// MonoBehaviour : nécessaire pour Update() (gestion des commandes différées + raccourcis clavier)
// et pour DontDestroyOnLoad()
public class CommandDispatcher : MonoBehaviour
{
    // ════════════════════════════════════════════════════════════════════════
    //   SINGLETON
    // ════════════════════════════════════════════════════════════════════════

    private static CommandDispatcher _instance; // L'unique instance du dispatcher

    // Propriété d'accès avec création automatique si nécessaire (lazy singleton)
    public static CommandDispatcher Instance
    {
        get
        {
            if (_instance == null) CreateInstance(); // Crée l'instance si elle n'existe pas encore
            return _instance;
        }
    }


    // ════════════════════════════════════════════════════════════════════════
    //   PILES UNDO / REDO
    // ════════════════════════════════════════════════════════════════════════

    // Stack = structure de données LIFO (Last In, First Out = dernier entré, premier sorti)
    // Parfait pour Undo : on annule toujours la dernière action effectuée
    //
    // _undoStack : historique des actions exécutées (prêtes à être annulées)
    // _redoStack : actions annulées (prêtes à être rétablies)
    private readonly Stack<ICommand> _undoStack = new Stack<ICommand>();
    private readonly Stack<ICommand> _redoStack = new Stack<ICommand>();

    // Taille maximale de l'historique Undo
    // [SerializeField] : rend ce champ modifiable dans l'Inspector Unity
    // Utile pour ajuster la limite sans modifier le code
    [SerializeField] private int _maxHistorySize = 50;

    // Liste des commandes programmées pour une exécution future
    // List et non Queue car on parcourt en ordre inverse pour retirer les éléments efficacement
    private readonly List<ScheduledCommand> _scheduled = new List<ScheduledCommand>();


    // ════════════════════════════════════════════════════════════════════════
    //   ÉVÉNEMENTS C# — Notification externe des actions
    // ════════════════════════════════════════════════════════════════════════

    // Ces événements permettent à d'autres systèmes (UI, logs, achievements...)
    // d'être notifiés quand des actions sont exécutées/annulées/rétablies
    // 'event' : mot-clé C# qui empêche les abonnés externes d'invoquer l'événement directement
    public static event Action<ICommand> OnCommandExecuted; // Déclenché après chaque Execute()
    public static event Action<ICommand> OnCommandUndone;   // Déclenché après chaque Undo()
    public static event Action<ICommand> OnCommandRedone;   // Déclenché après chaque Redo()


    // ════════════════════════════════════════════════════════════════════════
    //   INITIALISATION AUTOMATIQUE
    // ════════════════════════════════════════════════════════════════════════

    [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.BeforeSceneLoad)]
    // Appelé automatiquement par Unity avant le chargement de la première scène
    private static void CreateInstance()
    {
        if (_instance != null) return; // Ne crée pas deux instances

        var go = new GameObject("[CommandDispatcher]"); // Crée le GameObject porteur
        _instance = go.AddComponent<CommandDispatcher>(); // Attache le script
        DontDestroyOnLoad(go); // Persiste entre les changements de scène
    }


    // ════════════════════════════════════════════════════════════════════════
    //   MÉTHODE : Execute — Exécuter une commande immédiatement
    // ════════════════════════════════════════════════════════════════════════

    public static void Execute(ICommand command)
    {
        if (command == null) return; // Sécurité : ignore les commandes nulles

        command.Execute(); // ← Appelle la méthode Execute() de la commande
                           //   C'est ici que l'action se produit réellement dans le jeu

        // Vérifie si la commande supporte l'annulation (implémente IUndoableCommand)
        // 'is IUndoableCommand undoable' : pattern matching C# 7 — cast + vérification en une ligne
        if (command is IUndoableCommand undoable)
        {
            Instance._undoStack.Push(undoable); // Empile la commande sur l'historique Undo

            Instance._redoStack.Clear(); // IMPORTANT : on efface le Redo stack
                                         // Logique : si on fait A → B → Undo B → fait C,
                                         // on ne peut plus "Redo B" car on est sur une nouvelle branche

            // Vérifie si l'historique dépasse la limite autorisée
            while (Instance._undoStack.Count > Instance._maxHistorySize)
                Instance.TrimUndoStack(); // Retire la commande la plus ancienne
        }

        // Notifie tous les abonnés à OnCommandExecuted (UI, logs, etc.)
        // '?.' : null-conditionnel — si personne n'est abonné, ne lève pas d'exception
        OnCommandExecuted?.Invoke(command);
    }


    // ════════════════════════════════════════════════════════════════════════
    //   MÉTHODE : Undo — Annuler la dernière action
    // ════════════════════════════════════════════════════════════════════════

    public static bool Undo()
    {
        if (Instance._undoStack.Count == 0) // Vérifie qu'il y a quelque chose à annuler
        {
            Debug.Log("[CommandDispatcher] Rien à annuler.");
            return false; // Retourne false → l'appelant sait que l'undo a échoué
        }

        // Pop() : retire ET retourne l'élément du sommet de la pile (la dernière commande exécutée)
        // Cast vers IUndoableCommand pour accéder à la méthode Undo()
        var cmd = Instance._undoStack.Pop() as IUndoableCommand;

        cmd?.Undo(); // Appelle Undo() sur la commande → inverse l'action dans le jeu

        Instance._redoStack.Push(cmd); // Déplace la commande vers le stack Redo
                                        // → elle pourra être rétablie avec Redo()

        OnCommandUndone?.Invoke(cmd); // Notifie les abonnés (ex: griser le bouton Undo dans l'UI)
        return true;
    }


    // ════════════════════════════════════════════════════════════════════════
    //   MÉTHODE : Redo — Rétablir la dernière action annulée
    // ════════════════════════════════════════════════════════════════════════

    public static bool Redo()
    {
        if (Instance._redoStack.Count == 0) // Vérifie qu'il y a quelque chose à rétablir
        {
            Debug.Log("[CommandDispatcher] Rien à rétablir.");
            return false;
        }

        var cmd = Instance._redoStack.Pop(); // Récupère la dernière commande annulée

        cmd.Execute(); // Ré-exécute la commande (comme si on la refaisait)

        Instance._undoStack.Push(cmd); // Remet la commande dans l'Undo stack
                                        // → elle peut à nouveau être annulée

        OnCommandRedone?.Invoke(cmd); // Notifie les abonnés
        return true;
    }


    // ════════════════════════════════════════════════════════════════════════
    //   MÉTHODE : Schedule — Programmer une commande dans le futur
    // ════════════════════════════════════════════════════════════════════════

    public static void Schedule(ICommand command, float delaySeconds)
    {
        // Ajoute une ScheduledCommand à la liste
        // ExecuteAt = l'instant précis (en secondes depuis le démarrage du jeu) où exécuter
        Instance._scheduled.Add(new ScheduledCommand
        {
            Command = command,
            ExecuteAt = Time.time + delaySeconds // Time.time = secondes écoulées depuis le lancement
        });
    }


    // ════════════════════════════════════════════════════════════════════════
    //   MÉTHODE : ExecuteBatch — Exécuter plusieurs commandes comme UNE seule
    // ════════════════════════════════════════════════════════════════════════

    // 'params ICommand[]' : accepte un nombre variable d'arguments
    // Ex: ExecuteBatch(cmdA, cmdB, cmdC) — sans créer de tableau manuellement
    public static void ExecuteBatch(params ICommand[] commands)
    {
        // Encapsule toutes les commandes dans une BatchCommand
        // BatchCommand est elle-même une IUndoableCommand → 1 seul Undo pour toutes
        Execute(new BatchCommand(commands));
    }


    // ════════════════════════════════════════════════════════════════════════
    //   PROPRIÉTÉS D'ÉTAT — Consulter l'état du dispatcher
    // ════════════════════════════════════════════════════════════════════════

    // Expression body properties (=>) : propriétés calculées en une ligne
    public static bool CanUndo  => Instance._undoStack.Count > 0; // Y a-t-il des actions à annuler ?
    public static bool CanRedo  => Instance._redoStack.Count > 0; // Y a-t-il des actions à rétablir ?
    public static int  UndoCount => Instance._undoStack.Count;    // Nombre d'actions annulables

    // Vide complètement les deux historiques
    public static void ClearHistory()
    {
        Instance._undoStack.Clear(); // Vide l'historique Undo
        Instance._redoStack.Clear(); // Vide l'historique Redo
    }


    // ════════════════════════════════════════════════════════════════════════
    //   MÉTHODE : Update — Gestion des commandes différées + raccourcis clavier
    // ════════════════════════════════════════════════════════════════════════

    private void Update()
    {
        // Parcourt la liste en ordre INVERSE pour pouvoir supprimer des éléments pendant l'itération
        // Si on parcourait en ordre normal et supprimait l'élément i, l'élément i+1 deviendrait i
        // → on raterait des éléments. En inverse, la suppression ne perturbe pas les indices non encore visités
        for (int i = _scheduled.Count - 1; i >= 0; i--)
        {
            if (Time.time >= _scheduled[i].ExecuteAt) // L'heure d'exécution est arrivée
            {
                Execute(_scheduled[i].Command); // Exécute la commande normalement
                _scheduled.RemoveAt(i);         // Retire de la liste (déjà exécutée)
            }
        }

        // Raccourci Ctrl+Z (Windows) ou Cmd+Z (Mac) → Undo
        if (Input.GetKeyDown(KeyCode.Z) && (Input.GetKey(KeyCode.LeftControl) || Input.GetKey(KeyCode.LeftCommand)))
            Undo();

        // Raccourci Ctrl+Y (Windows) ou Cmd+Y (Mac) → Redo
        if (Input.GetKeyDown(KeyCode.Y) && (Input.GetKey(KeyCode.LeftControl) || Input.GetKey(KeyCode.LeftCommand)))
            Redo();
    }


    // ════════════════════════════════════════════════════════════════════════
    //   MÉTHODE INTERNE : TrimUndoStack — Limiter la taille de l'historique
    // ════════════════════════════════════════════════════════════════════════

    private void TrimUndoStack()
    {
        // Stack n'offre pas d'accès par index, donc on doit contourner :
        var arr = _undoStack.ToArray(); // Convertit la pile en tableau
                                        // arr[0] = sommet (le plus récent), arr[Last] = le plus ancien

        _undoStack.Clear(); // Vide la pile

        // Recrée la pile SANS le dernier élément (le plus ancien = arr[Length-1])
        // On repart de arr[Length-2] jusqu'à 0 pour conserver l'ordre
        for (int i = arr.Length - 2; i >= 0; i--)
            _undoStack.Push(arr[i]); // Re-empile dans le bon ordre
    }

    private void OnDestroy()
    {
        if (_instance == this) _instance = null; // Nettoie la référence statique à la destruction
    }


    // ════════════════════════════════════════════════════════════════════════
    //   CLASSE INTERNE : ScheduledCommand
    // ════════════════════════════════════════════════════════════════════════

    // Structure de données interne pour stocker une commande + son heure d'exécution
    // 'private class' : uniquement accessible depuis CommandDispatcher
    private class ScheduledCommand
    {
        public ICommand Command;   // La commande à exécuter
        public float ExecuteAt;    // Timestamp Unity (Time.time) auquel l'exécuter
    }
}


// ════════════════════════════════════════════════════════════════════════════
//   INTERFACES — Contrats des commandes
// ════════════════════════════════════════════════════════════════════════════

// Contrat minimal : toute commande doit pouvoir s'exécuter
public interface ICommand
{
    void Execute(); // Effectue l'action
}

// Contrat étendu : commande réversible — peut s'exécuter ET s'annuler
// 'IUndoableCommand : ICommand' = hérite de ICommand → doit aussi implémenter Execute()
public interface IUndoableCommand : ICommand
{
    void Undo(); // Inverse l'action (remet l'état comme avant Execute())
}


// ════════════════════════════════════════════════════════════════════════════
//   CLASSE : BatchCommand — Groupe de commandes atomique
// ════════════════════════════════════════════════════════════════════════════

// Implémente IUndoableCommand → peut être annulée d'un seul coup
public class BatchCommand : IUndoableCommand
{
    // Tableau de toutes les commandes du groupe
    // 'readonly' : le tableau ne peut pas être remplacé après construction
    private readonly ICommand[] _commands;

    // Constructeur : reçoit le tableau de commandes à grouper
    public BatchCommand(ICommand[] commands) => _commands = commands;

    // Execute toutes les commandes dans l'ordre d'ajout
    public void Execute()
    {
        foreach (var cmd in _commands) cmd.Execute();
    }

    // Undo en ordre INVERSE
    // Si Execute a fait A puis B, Undo doit défaire B puis A (ordre logique)
    public void Undo()
    {
        for (int i = _commands.Length - 1; i >= 0; i--)
            (_commands[i] as IUndoableCommand)?.Undo(); // Cast null-safe : si la commande n'est pas undoable, on saute
    }
}


// ════════════════════════════════════════════════════════════════════════════
//   COMMANDES EXEMPLES
// ════════════════════════════════════════════════════════════════════════════

// ── MoveCommand : déplace un Transform (réversible) ──────────────────────────
public class MoveCommand : IUndoableCommand
{
    private readonly Transform _target;      // L'objet à déplacer
    private readonly Vector3 _newPosition;   // Position APRÈS le déplacement
    private readonly Vector3 _oldPosition;   // Position AVANT le déplacement (sauvegardée pour Undo)

    // Le constructeur capture l'ancienne position AU MOMENT de la création
    // (pas au moment de l'exécution — important pour que Undo retrouve le bon état)
    public MoveCommand(Transform target, Vector3 newPosition)
    {
        _target = target;
        _newPosition = newPosition;
        _oldPosition = target.position; // Sauvegarde la position actuelle avant tout changement
    }

    public void Execute() => _target.position = _newPosition; // Applique la nouvelle position
    public void Undo()    => _target.position = _oldPosition; // Restaure l'ancienne position
}


// ── SetActiveCommand : active/désactive un GameObject (réversible) ────────────
public class SetActiveCommand : IUndoableCommand
{
    private readonly GameObject _target;
    private readonly bool _newState; // L'état APRÈS la commande
    private readonly bool _oldState; // L'état AVANT la commande (sauvegardé pour Undo)

    public SetActiveCommand(GameObject target, bool active)
    {
        _target = target;
        _newState = active;
        _oldState = target.activeSelf; // Sauvegarde l'état actuel
    }

    public void Execute() => _target.SetActive(_newState); // Active ou désactive
    public void Undo()    => _target.SetActive(_oldState); // Restaure l'état précédent
}


// ── SpawnCommand : instancie un prefab (NON réversible) ───────────────────────
// Implémente seulement ICommand (pas IUndoableCommand)
// → Elle n'ira pas dans l'UndoStack car on ne peut pas "dé-spawner" de façon générique
// (on pourrait implémenter un Undo qui détruit l'objet, mais c'est à vous de décider)
public class SpawnCommand : ICommand
{
    private readonly GameObject _prefab;   // Le prefab à instancier
    private readonly Vector3 _position;    // Où l'instancier dans le monde

    public SpawnCommand(GameObject prefab, Vector3 position)
    {
        _prefab = prefab;
        _position = position;
    }

    // Object.Instantiate : méthode statique Unity qui crée une copie du prefab dans la scène
    // Quaternion.identity = rotation nulle (pas de rotation initiale)
    public void Execute() => UnityEngine.Object.Instantiate(_prefab, _position, Quaternion.identity);
}


// ════════════════════════════════════════════════════════════════════════════
//   EXEMPLE D'UTILISATION (mis en commentaire)
// ════════════════════════════════════════════════════════════════════════════

/*
public class LevelEditor : MonoBehaviour
{
    public GameObject selectedObject;
    public GameObject spawnPrefab;

    private void Update()
    {
        if (Input.GetMouseButtonDown(0))
        {
            var newPos = Camera.main.ScreenToWorldPoint(Input.mousePosition);
            // Exécute et stocke dans UndoStack → Ctrl+Z ramènera l'objet à sa position précédente
            CommandDispatcher.Execute(new MoveCommand(selectedObject.transform, newPos));
        }

        if (Input.GetKeyDown(KeyCode.Space))
        {
            // Les deux actions sont traitées comme UNE par le système Undo
            // Ctrl+Z annulera SetActive ET le Spawn en une seule frappe
            CommandDispatcher.ExecuteBatch(
                new SetActiveCommand(selectedObject, false),
                new SpawnCommand(spawnPrefab, selectedObject.transform.position)
            );
        }

        if (Input.GetKeyDown(KeyCode.T))
            // Sera exécutée automatiquement dans 3 secondes par Update()
            CommandDispatcher.Schedule(new SpawnCommand(spawnPrefab, Vector3.zero), 3f);
    }
}
*/
