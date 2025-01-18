<h1>
PrimeShield - Projet de Sécurité des Données
</h1>

<div align="center">
        <img src="https://img.shields.io/badge/Rust-dea584?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
    <img src="https://img.shields.io/badge/Version-1.0.0-informational?style=for-the-badge" alt="Version" />
</div>

PrimeShield est un projet axé sur la sécurité des données, mettant en œuvre des primitives cryptographiques classiques.
Ce mini-projet utilise **Rust**, un langage moderne et performant, pour garantir un code sûr, fiable et optimisé.

Le projet explore des concepts clés de la cryptographie mathématique tels que l'exponentiation rapide, les tests de
primalité, la génération de nombres premiers, l'inverse modulaire et la construction d'une procédure complète de
signature RSA.

👉 Retrouvez l'exécutable pour votre système d'exploitation dans la
section [Releases](https://github.com/Maxime-Cllt/PrimeShield/releases).


<div align="center" style="display: flex; justify-content: space-around;">
    <img src="/assets/app1.png" alt="PrimeShield" width="400px" height="300px" />
    <img src="/assets/app2.png" alt="PrimeShield" width="400px" height="300px" />
</div>

---

## 📝 Description

PrimeShield implémente des primitives cryptographiques essentielles utilisées dans des protocoles comme **RSA**, avec
une attention particulière portée sur la sécurité mathématique.

Les principales fonctionnalités du projet incluent :

1. **Exponentiation rapide** : Calcul efficace de \( g^x \mod n \).
2. **Test de primalité** : Vérification probabiliste si un nombre est premier.
3. **Primalité relative** : Utilisation de l'algorithme d'Euclide pour déterminer si deux entiers sont premiers entre
   eux.
4. **Génération de nombres premiers** : Génération aléatoire de nombres premiers inférieurs à une valeur donnée.
5. **Inverse modulaire** : Calcul de l'inverse modulaire, une étape cruciale pour la signature RSA et
   l'authentification.

### 🚀 Objectif

L'objectif principal est de démontrer comment les primitives cryptographiques peuvent être mises en œuvre efficacement
pour sécuriser les communications numériques.

---

## 💻 Plateformes Compatibles

PrimeShield est compatible avec les systèmes d'exploitation suivants :

<div align="center">
    <img src="https://img.shields.io/badge/OS-MacOS-informational?style=flat&logo=apple&logoColor=white&color=007aff" alt="MacOS" />
    <img src="https://img.shields.io/badge/OS-Linux-informational?style=flat&logo=linux&logoColor=white&color=ff7f00" alt="Linux" />
    <img src="https://img.shields.io/badge/OS-Windows-informational?style=flat&logo=windows&logoColor=white&color=1e90ff" alt="Windows" />
</div>

---

## ⚙️ Pré-requis

Avant de commencer, assurez-vous d’avoir les éléments suivants installés :

- **Rust** : Version 1.83 ou supérieure.
- **Cargo** : Gestionnaire de dépendances et outil de compilation pour Rust.

<div align="center">
    <img src="https://img.shields.io/badge/Rust-1.83+-informational?style=flat&logo=rust&logoColor=white&color=53a863" alt="Rust" />
    <img src="https://img.shields.io/badge/Cargo-informational?style=flat&logo=rust&logoColor=white&color=53a863" alt="Cargo" />
</div>

👉 Si Rust et Cargo ne sont pas installés,
consultez [la documentation officielle](https://www.rust-lang.org/tools/install) pour les installer.

---

## 📦 Installation

Suivez ces étapes pour configurer et exécuter PrimeShield :

1. **Clonez le dépôt :**

```bash
git clone https://github.com/Maxime-Cllt/PrimeShield.git
cd PrimeShield
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
.\target\release\PrimeShield.exe
```