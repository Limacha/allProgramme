#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>

#define PORT 4444
#define BUFFER_SIZE 1024

int LaunchServer() {
    int server_fd, client_fd;
    struct sockaddr_in server_addr, client_addr;
    socklen_t addr_len = sizeof(client_addr);
    char buffer[BUFFER_SIZE];

    // Création du socket
    server_fd = socket(AF_INET, SOCK_STREAM, 0);
    if (server_fd == -1) {
        perror("Erreur socket");
        exit(1);
    }

    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = INADDR_ANY;
    server_addr.sin_port = htons(PORT);

    // Liaison du socket
    if (bind(server_fd, (struct sockaddr*)&server_addr, sizeof(server_addr)) == -1) {
        perror("Erreur bind");
        close(server_fd);
        exit(1);
    }

    // Écoute des connexions
    if (listen(server_fd, 1) == -1) {
        perror("Erreur listen");
        close(server_fd);
        exit(1);
    }

    printf("En attente de connexion...\n");

    // Accepter la connexion d'un client
    client_fd = accept(server_fd, (struct sockaddr*)&client_addr, &addr_len);
    if (client_fd == -1) {
        perror("Erreur accept");
        close(server_fd);
        exit(1);
    }

    printf("Client connecté !\n");

    while (1) {
        printf("Commande à exécuter : ");
        fgets(buffer, BUFFER_SIZE, stdin);
        buffer[strcspn(buffer, "\n")] = 0;  // Supprimer le \n

        send(client_fd, buffer, strlen(buffer), 0);

        memset(buffer, 0, BUFFER_SIZE);
        recv(client_fd, buffer, BUFFER_SIZE, 0);
        printf("Résultat :\n%s\n", buffer);
    }

    close(client_fd);
    close(server_fd);
    return 0;
}
