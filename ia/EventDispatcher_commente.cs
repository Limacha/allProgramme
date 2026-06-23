// ── Importation des espaces de noms nécessaires ──────────────────────────────

using System;                    // Fournit : Type, Action<T>, Delegate — types de base du C#
using System.Collections.Generic; // Fournit : Dictionary<K,V>, List<T> — collections génériques
using UnityEngine;               // Fournit : Vector3, GameObject — types Unity

// ── Déclaration de la classe ──────────────────────────────────────────────────

// 'public'  : accessible depuis n'importe quel script du projet
// 'static'  : la classe n'est jamais instanciée, on appelle directement EventDispatcher.Subscribe(...)
//             Il n'y a pas besoin de faire "new EventDispatcher()"
public static class EventDispatcher
{
    // ── Stockage central des abonnés ──────────────────────────────────────────

    // Dictionary<Type, List<Delegate>> :
    //   Clé   = le TYPE de l'événement (ex: typeof(PlayerDiedEvent))
    //   Valeur = la LISTE de toutes les méthodes abonnées à cet événement
    //
    // 'private'  : seul le dispatcher lui-même peut y accéder
    // 'static'   : une seule copie partagée pour tout le jeu (données de classe, pas d'instance)
    // 'readonly' : la référence au dictionnaire ne peut pas être remplacée après initialisation
    //              (on peut quand même ajouter/retirer des entrées à l'intérieur)
    private static readonly Dictionary<Type, List<Delegate>> _handlers =
        new Dictionary<Type, List<Delegate>>();  // Initialisé à vide au démarrage


    // ════════════════════════════════════════════════════════════════════════
    //   MÉTHODE : Subscribe — S'abonner à un type d'événement
    // ════════════════════════════════════════════════════════════════════════

    // '<T>'           : générique — T est le type de l'événement (ex: PlayerDiedEvent)
    // 'Action<T>'     : un délégué (pointeur de fonction) qui accepte un T en paramètre
    // 'where T:struct': contrainte — T doit être une struct (valeur légère, pas de GC)
    public static void Subscribe<T>(Action<T> handler) where T : struct
    {
        var type = typeof(T); // Récupère le Type C# correspondant à T (ex: typeof(PlayerDiedEvent))
                              // On s'en sert comme clé dans le dictionnaire

        // Vérifie si ce type d'événement a déjà une liste d'abonnés
        // Si non → on crée une nouvelle liste vide pour lui
        if (!_handlers.ContainsKey(type))
            _handlers[type] = new List<Delegate>();

        // Vérifie que ce handler n'est pas déjà dans la liste
        // Évite les doublons si Subscribe est appelé deux fois par erreur
        if (!_handlers[type].Contains(handler))
            _handlers[type].Add(handler); // Ajoute le handler à la liste
    }


    // ════════════════════════════════════════════════════════════════════════
    //   MÉTHODE : Unsubscribe — Se désabonner d'un type d'événement
    // ════════════════════════════════════════════════════════════════════════

    public static void Unsubscribe<T>(Action<T> handler) where T : struct
    {
        var type = typeof(T); // Récupère le type de l'événement (même logique que Subscribe)

        // Vérifie que ce type d'événement existe dans le dictionnaire
        // Si l'événement n'a jamais eu d'abonné, inutile de chercher
        if (_handlers.ContainsKey(type))
            _handlers[type].Remove(handler); // Retire le handler de la liste
                                             // Si handler n'existe pas, Remove ne fait rien (pas d'erreur)
    }


    // ════════════════════════════════════════════════════════════════════════
    //   MÉTHODE : Publish — Déclencher un événement (notifier tous les abonnés)
    // ════════════════════════════════════════════════════════════════════════

    // 'T eventData' : les données de l'événement passées à tous les abonnés
    //                 (ex: new PlayerDiedEvent { Score = 42 })
    public static void Publish<T>(T eventData) where T : struct
    {
        var type = typeof(T); // Récupère le type pour trouver la bonne liste dans le dictionnaire

        // Si personne n'est abonné à ce type d'événement → on sort immédiatement
        // 'return' sans valeur car la méthode est void
        if (!_handlers.ContainsKey(type)) return;

        // SÉCURITÉ IMPORTANTE : on crée une copie (snapshot) de la liste avant d'itérer
        // Pourquoi ? Si un handler appelle Unsubscribe() pendant l'itération,
        // il modifierait la liste en cours de parcours → exception InvalidOperationException
        // En itérant sur la copie, la liste originale peut changer sans problème
        var snapshot = new List<Delegate>(_handlers[type]);

        // Parcourt tous les handlers abonnés et les appelle un par un
        foreach (var handler in snapshot)
            // 'handler as Action<T>' : cast du Delegate générique vers Action<T> spécifique
            // '?.'                   : opérateur null-conditionnel — si le cast échoue (null), ne fait rien
            // '.Invoke(eventData)'   : appelle réellement la méthode avec les données de l'événement
            (handler as Action<T>)?.Invoke(eventData);
    }


    // ════════════════════════════════════════════════════════════════════════
    //   MÉTHODE : Clear — Nettoyer les abonnés
    // ════════════════════════════════════════════════════════════════════════

    // Supprime TOUS les abonnés de TOUS les événements
    // Utile lors d'un changement de scène pour éviter les références mortes
    public static void Clear()
    {
        _handlers.Clear(); // Vide complètement le dictionnaire
    }

    // Surcharge générique : supprime uniquement les abonnés d'un type d'événement précis
    // Ex: EventDispatcher.Clear<PlayerDiedEvent>() → ne supprime que les abonnés à PlayerDiedEvent
    public static void Clear<T>() where T : struct
    {
        _handlers.Remove(typeof(T)); // Retire l'entrée du dictionnaire pour ce type
    }
}


// ════════════════════════════════════════════════════════════════════════════
//   DÉFINITION DES ÉVÉNEMENTS (structs)
// ════════════════════════════════════════════════════════════════════════════

// On utilise des 'struct' et non des 'class' pour deux raisons :
//   1. Légèreté : alloués sur la pile (stack), pas sur le tas (heap) → pas de garbage collector
//   2. La contrainte 'where T : struct' dans le dispatcher l'exige

// Événement : le joueur est mort
public struct PlayerDiedEvent
{
    public int Score;        // Le score final du joueur au moment de sa mort
    public Vector3 Position; // La position dans le monde où il est mort
}

// Événement : un ennemi vient d'apparaître
public struct EnemySpawnedEvent
{
    public GameObject Enemy;  // Référence au GameObject de l'ennemi spawné
    public int WaveIndex;     // Numéro de la vague à laquelle il appartient
}

// Événement : la partie est terminée
public struct GameOverEvent
{
    public bool Victory; // true = victoire, false = défaite
}


// ════════════════════════════════════════════════════════════════════════════
//   EXEMPLE D'UTILISATION (mis en commentaire, ne pas décommenter tel quel)
// ════════════════════════════════════════════════════════════════════════════

/*
public class PlayerHealth : MonoBehaviour
{
    private void OnEnable()
    {
        // OnEnable est appelé quand le GameObject devient actif
        // On s'abonne ici (et pas dans Start) pour gérer les objets réactivés
        EventDispatcher.Subscribe<EnemySpawnedEvent>(OnEnemySpawned);
    }

    private void OnDisable()
    {
        // CRITIQUE : toujours se désabonner dans OnDisable ou OnDestroy
        // Sinon : si le GameObject est détruit, le handler pointe vers un objet mort
        // → NullReferenceException lors du prochain Publish
        EventDispatcher.Unsubscribe<EnemySpawnedEvent>(OnEnemySpawned);
    }

    private void Die()
    {
        // Publie l'événement avec ses données
        // Tous les abonnés à PlayerDiedEvent seront notifiés immédiatement (synchrone)
        EventDispatcher.Publish(new PlayerDiedEvent
        {
            Score = 100,
            Position = transform.position  // Capture la position actuelle
        });
    }

    // Méthode callback : appelée automatiquement quand EnemySpawnedEvent est publié
    private void OnEnemySpawned(EnemySpawnedEvent e)
    {
        Debug.Log($"Ennemi vague {e.WaveIndex} apparu !");
    }
}
*/
