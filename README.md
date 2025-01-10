# PrimeShield - Projet de Sécurité des Données

PrimeShield est un projet de sécurité des données utilisant les primitives cryptographiques classiques. Le but de ce
mini-projet est de mettre en œuvre plusieurs éléments de la cryptographie, comme l'exponentiation rapide, le test de
primalité, la génération de nombres premiers, l'inverse modulaire, et de construire une procédure de signature RSA
complète. L'implémentation est réalisée en Rust, un langage de programmation moderne qui garantit la
sécurité et la fiabilité du code, tout en apportant une haute performance.

Retrouvez un exécutable du programme pour votre système d'exploitation dans la
section [Releases](https://github.com/Maxime-Cllt/PrimeShield/releases).

## Description

Ce projet implémente des primitives cryptographiques utilisées dans des protocoles comme RSA, avec un accent particulier
sur les tests de primalité et l'inverse modulaire. L'objectif est de démontrer l'utilisation des méthodes mathématiques
pour garantir la sécurité des communications.

Les principales étapes incluent :

1. **Exponentiation rapide** : Calcul de \( g^x \mod n \) pour trois entiers donnés \( g \), \( x \) et \( n \).
2. **Test de primalité** : Vérification probabiliste si un nombre \( p \) est premier.
3. **Test de primalité relative** : Utilisation de l'algorithme d'Euclide pour déterminer si deux entiers sont premiers
   entre eux.
4. **Génération de nombres premiers** : Génération d'un nombre premier aléatoire inférieur à une certaine valeur \(
   n \).
5. **Inverse modulaire** : Calcul de l'inverse modulaire utilisé dans le protocole RSA pour la signature et l'
   authentification.

## Plateformes

PrimeShield fonctionne sur les plateformes suivantes :

<div align="center">
    <img src="https://img.shields.io/badge/OS-MacOS-informational?style=flat&logo=apple&logoColor=white&color=007aff" alt="MacOS" />
    <img src="https://img.shields.io/badge/OS-Linux-informational?style=flat&logo=linux&logoColor=white&color=ff7f00" alt="Linux" />
    <img src="https://img.shields.io/badge/OS-Windows-informational?style=flat&logo=windows&logoColor=white&color=1e90ff" alt="Windows" />
</div>

### Pré-requis

Le projet nécessite les outils suivants :

- **Rust** : Version 1.83 ou supérieure.
- **Cargo** : Outil de gestion de dépendances et de compilation pour Rust.

<div align="center">
<img src="https://img.shields.io/badge/Rust-1.83+-informational?style=flat&logo=rust&logoColor=white&color=53a863" alt="Rust" />
<img src="https://img.shields.io/badge/Cargo-informational?style=flat&logo=rust&logoColor=white&color=53a863" alt="Cargo" />
</div>

## Installation

Pour exécuter le programme, vous devez avoir Rust et Cargo installés sur votre machine. Si ce n'est pas le cas, vous
pouvez les installer en suivant les instructions sur le site officiel de Rust.

1. Clonez le dépôt :

```bash
git clone https://github.com/Maxime-Cllt/PrimeShield.git
```

2. Compiler le programme:

```bash
cargo build --release
```

3. Exécutez le programme:

Il se peut que vous ayez besoin de donner les permissions d'exécution au fichier binaire généré.

```bash
chmod +x target/release/PrimeShield
```

### MacOS & Linux

```bash
./target/release/PrimeShield
```

### Windows

```bash
./target/release/PrimeShield.exe
```