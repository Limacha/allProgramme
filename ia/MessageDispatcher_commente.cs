// ── Importation des espaces de noms nécessaires ──────────────────────────────

using System;                    // Fournit : Type, Action<T>, Delegate
using System.Collections.Generic; // Fournit : Dictionary<K,V>, List<T>
using UnityEngine;               // Fournit : GameObject — clé centrale du dispatcher

// ── Déclaration de la classe ──────────────────────────────────────────────────

// 'static' : pas d'instanciation, appels directs — MessageDispatcher.Send(...)
public static class MessageDispatcher
{
    // ════════════════════════════════════════════════════════════════════════
    //   STOCKAGE CENTRAL — Récepteurs ciblés par GameObject
    // ════════════════════════════════════════════════════════════════════════

    // Structure imbriquée à 3 niveaux :
    //   Niveau 1 — GameObject  : quel objet reçoit le message ?
    //   Niveau 2 — Type        : quel type de message ? (ex: AttackMessage)
    //   Niveau 3 — List<Delegate> : quelles méthodes réagissent à ce message sur cet objet ?
    //
    // Exemple concret :
    //   _receivers[enemyGO][typeof(AttackMessage)] = [OnAttacked_Shield, OnAttacked_Health]
    //   _receivers[playerGO][typeof(HealMessage)]  = [OnHealed]
    private static readonly Dictionary<GameObject, Dictionary<Type, List<Delegate>>> _receivers =
        new Dictionary<GameObject, Dictionary<Type, List<Delegate>>>();

    // Récepteurs globaux pour le broadcast (pas liés à un GameObject spécifique)
    // Structure à 2 niveaux : Type → List<Delegate>
    // Tous les objets qui ont appelé RegisterBroadcast<T> sont ici
    private static readonly Dictionary<Type, List<Delegate>> _broadcastReceivers =
        new Dictionary<Type, List<Delegate>>();


    // ════════════════════════════════════════════════════════════════════════
    //   MÉTHODE : Register — Enregistrer un récepteur ciblé
    // ════════════════════════════════════════════════════════════════════════

    // 'T'              : le type du message (doit implémenter IMessage)
    // 'target'         : le GameObject qui veut recevoir ce type de message
    // 'handler'        : la méthode à appeler quand le message arrive
    // 'where T:IMessage' : contrainte — T doit implémenter l'interface IMessage
    public static void Register<T>(GameObject target, Action<T> handler) where T : IMessage
    {
        if (target == null || handler == null) return; // Sécurité : refuse les arguments invalides

        // Vérifie si ce GameObject a déjà une entrée dans le dictionnaire principal
        // Si non → crée un nouveau sous-dictionnaire pour lui
        if (!_receivers.ContainsKey(target))
            _receivers[target] = new Dictionary<Type, List<Delegate>>();

        var typeDict = _receivers[target]; // Raccourci vers le sous-dictionnaire de cet objet
        var type = typeof(T);              // Le type du message (sera la clé du sous-dictionnaire)

        // Vérifie si ce type de message a déjà une liste pour cet objet
        // Si non → crée une liste vide
        if (!typeDict.ContainsKey(type))
            typeDict[type] = new List<Delegate>();

        // Vérifie qu'on n'ajoute pas deux fois le même handler (protection contre les doublons)
        if (!typeDict[type].Contains(handler))
            typeDict[type].Add(handler); // Ajoute le handler à la liste
    }

    // Enregistrement pour le broadcast global (sans GameObject spécifique)
    public static void RegisterBroadcast<T>(Action<T> handler) where T : IMessage
    {
        var type = typeof(T); // Type du message

        if (!_broadcastReceivers.ContainsKey(type))
            _broadcastReceivers[type] = new List<Delegate>(); // Crée la liste si elle n'existe pas

        if (!_broadcastReceivers[type].Contains(handler))
            _broadcastReceivers[type].Add(handler); // Ajoute le handler global
    }


    // ════════════════════════════════════════════════════════════════════════
    //   MÉTHODES : Unregister — Se désenregistrer
    // ════════════════════════════════════════════════════════════════════════

    // Retire UN handler spécifique pour UN type de message sur UN GameObject
    public static void Unregister<T>(GameObject target, Action<T> handler) where T : IMessage
    {
        if (target == null || !_receivers.ContainsKey(target)) return; // Sécurité : objet inexistant

        var type = typeof(T);
        if (_receivers[target].ContainsKey(type))
            _receivers[target][type].Remove(handler); // Retire le handler précis
    }

    // Retire TOUS les handlers de TOUS les types pour un GameObject donné
    // Utile dans OnDestroy pour nettoyer d'un coup toutes les inscriptions d'un objet
    public static void UnregisterAll(GameObject target)
    {
        if (target != null)
            _receivers.Remove(target); // Supprime toute l'entrée du dictionnaire pour cet objet
    }

    // Retire un handler du broadcast global
    public static void UnregisterBroadcast<T>(Action<T> handler) where T : IMessage
    {
        var type = typeof(T);
        if (_broadcastReceivers.ContainsKey(type))
            _broadcastReceivers[type].Remove(handler);
    }


    // ════════════════════════════════════════════════════════════════════════
    //   MÉTHODE : Send — Envoyer un message à un GameObject précis
    // ════════════════════════════════════════════════════════════════════════

    // Retourne 'bool' : true si au moins un handler a été appelé, false sinon
    // Permet à l'expéditeur de savoir si son message a été reçu
    public static bool Send<T>(GameObject target, T message) where T : IMessage
    {
        // Vérification 1 : l'objet existe et est actif dans la hiérarchie
        // Pas de message envoyé à un objet désactivé ou détruit
        if (target == null || !target.activeInHierarchy) return false;

        // Vérification 2 : l'objet a bien des handlers enregistrés
        if (!_receivers.ContainsKey(target)) return false;

        var type = typeof(T); // Type du message pour trouver les bons handlers

        // Vérification 3 : l'objet a des handlers pour CE type de message spécifiquement
        if (!_receivers[target].ContainsKey(type)) return false;

        // Snapshot de la liste pour éviter les modifications pendant l'itération
        // (même précaution que dans EventDispatcher)
        var snapshot = new List<Delegate>(_receivers[target][type]);

        bool handled = false; // Suivi : au moins un handler a-t-il été appelé ?

        foreach (var handler in snapshot)
        {
            (handler as Action<T>)?.Invoke(message); // Appelle le handler avec le message
            handled = true;

            // ── ANNULATION EN CASCADE ──────────────────────────────────────
            // Si le handler a posé message.Consumed = true,
            // on arrête immédiatement la propagation aux handlers suivants
            // Exemple : le bouclier absorbe l'attaque → la santé n'est pas touchée
            if (message.Consumed) break;
        }

        return handled; // Informe l'expéditeur si le message a été traité
    }


    // ════════════════════════════════════════════════════════════════════════
    //   MÉTHODE : Broadcast — Envoyer un message à TOUS les récepteurs globaux
    // ════════════════════════════════════════════════════════════════════════

    public static void Broadcast<T>(T message) where T : IMessage
    {
        var type = typeof(T);

        // Pas de récepteurs globaux pour ce type → rien à faire
        if (!_broadcastReceivers.ContainsKey(type)) return;

        var snapshot = new List<Delegate>(_broadcastReceivers[type]); // Copie de sécurité

        foreach (var handler in snapshot)
        {
            (handler as Action<T>)?.Invoke(message); // Notifie chaque récepteur global
            if (message.Consumed) break;             // Annulation en cascade possible aussi en broadcast
        }
    }


    // ════════════════════════════════════════════════════════════════════════
    //   MÉTHODE : Clear — Nettoyage complet
    // ════════════════════════════════════════════════════════════════════════

    public static void Clear()
    {
        _receivers.Clear();          // Efface tous les récepteurs ciblés
        _broadcastReceivers.Clear(); // Efface tous les récepteurs globaux
    }
}


// ════════════════════════════════════════════════════════════════════════════
//   INTERFACE IMessage — Contrat que tous les messages doivent respecter
// ════════════════════════════════════════════════════════════════════════════

// 'interface' : définit un contrat — toute classe qui implémente IMessage DOIT avoir 'Consumed'
// C'est ce qui permet le mécanisme d'annulation en cascade dans Send() et Broadcast()
public interface IMessage
{
    // Propriété que le handler peut mettre à 'true' pour stopper la propagation
    // get; set; → lisible ET modifiable
    bool Consumed { get; set; }
}


// ════════════════════════════════════════════════════════════════════════════
//   DÉFINITION DES MESSAGES (classes qui implémentent IMessage)
// ════════════════════════════════════════════════════════════════════════════

// Ici on utilise des 'class' (pas des structs) car les messages sont des objets mutables
// (on modifie Consumed à l'intérieur des handlers — impossible avec une struct passée par valeur)

// Message d'attaque : envoyé à un ennemi/joueur quand il reçoit des dégâts
public class AttackMessage : IMessage
{
    public bool Consumed { get; set; } // Implémentation requise par IMessage
    public float Damage;               // Quantité de dégâts infligés
    public GameObject Attacker;        // Référence à l'objet attaquant (pour savoir qui attaque)
    public DamageType Type;            // Type de dégâts (physique, magique, feu...)
}

// Message de soin
public class HealMessage : IMessage
{
    public bool Consumed { get; set; }
    public float Amount; // Quantité de points de vie restaurés
}

// Message de pause : broadcasté à tous les systèmes du jeu
public class GamePauseMessage : IMessage
{
    public bool Consumed { get; set; }
    public bool Paused; // true = le jeu se met en pause, false = reprend
}

// Énumération des types de dégâts (utilisée dans AttackMessage)
public enum DamageType { Physical, Magic, Fire, Ice }


// ════════════════════════════════════════════════════════════════════════════
//   EXEMPLE D'UTILISATION (mis en commentaire)
// ════════════════════════════════════════════════════════════════════════════

/*
public class Enemy : MonoBehaviour
{
    private float _health = 100f;

    private void OnEnable()
    {
        // Enregistre ce GameObject pour recevoir les AttackMessage qui lui sont envoyés
        MessageDispatcher.Register<AttackMessage>(gameObject, OnAttacked);
        // Enregistre en broadcast pour GamePauseMessage (tous les ennemis s'y abonnent)
        MessageDispatcher.RegisterBroadcast<GamePauseMessage>(OnGamePause);
    }

    private void OnDisable()
    {
        // TOUJOURS se désenregistrer pour éviter les appels sur un objet mort
        MessageDispatcher.Unregister<AttackMessage>(gameObject, OnAttacked);
        MessageDispatcher.UnregisterBroadcast<GamePauseMessage>(OnGamePause);
    }

    private void OnAttacked(AttackMessage msg)
    {
        _health -= msg.Damage;
        Debug.Log($"Ennemi touché ! HP restants : {_health}");
        msg.Consumed = true; // Stoppe la propagation → les handlers suivants ne seront pas appelés
    }

    private void OnGamePause(GamePauseMessage msg)
    {
        enabled = !msg.Paused; // Désactive/réactive le script selon l'état de pause
    }
}

// Depuis le script du joueur — envoie un message UNIQUEMENT à cet ennemi précis :
// MessageDispatcher.Send(enemyGO, new AttackMessage { Damage = 25f, Attacker = gameObject });

// Depuis le GameManager — envoie à TOUS les abonnés broadcast :
// MessageDispatcher.Broadcast(new GamePauseMessage { Paused = true });
*/
