use std::fmt;
use rand::prelude::*;
use std::iter::IntoIterator;
use rand::seq::SliceRandom;

fn main() {

    #[derive(Debug)]
    struct Environment<TS, TA, TR, TT, TP> {
        S: TS,
        A: TA,
        R: TR,
        T: TT,
        p: TP,
    }

    fn linear_world() -> Environment<TS, TA, TR, TT, TP> {
        // CREATION DE L'ENVIRONNEMENT LINE WORLD
        // ensemble des états possibles
        let mut S  = vec![0, 1, 2, 3, 4];
        // ensemble des actions possibles, O gauche, 1 droite
        let mut A= vec![0, 1];
        // ensemble des rewards possibles
        let mut R = vec![-1, 0, 1];
        // ensemble des états terminaux
        let mut T = vec![0, 4];
        // définition de p
        let mut p= vec![
            vec![
                vec![vec![0f32; R.len()]; S.len()];
                A.len()
            ];
            S.len()
        ];

        /*
    // affichage de p pour comprendre avec quoi on travaille
    for s in 0..S.len() {
        for a in 0..A.len() {
            for s_p in 0..S.len() {
                for r in 0..R.len() {
                    println!(
                        "p[{}][{}][{}][{}] = {}",
                        s, a, s_p, r, p[s][a][s_p][r]
                    );
                }
            }
        }
    }
    */

        // mise à jour de p
        for s in 0..S.len() {
            for a in 0..A.len() {
                for s_p in 0..S.len() {
                    for r in 0..R.len() {
                        if s_p == (s + 1) && a == 1 && R[r] == 0 && [1, 2].contains(&S[s]) {
                            p[s][a][s_p][r] = 1f32;
                        }
                        if s > 0 && s_p == (s - 1) && a == 0 && R[r] == 0 && [2, 3].contains(&S[s]) {
                            p[s][a][s_p][r] = 1f32;
                        }
                    }
                }
            }
        }

        p[3][1][4][2] = 1f32;
        p[1][0][0][0] = 1f32;
    /*
        // affichage de p pour comprendre avec quoi on travaille
        for s in 0..S.len() {
            for a in 0..A.len() {
                for s_p in 0..S.len() {
                    for r in 0..R.len() {
                        println!(
                            "p[{}][{}][{}][{}] = {}",
                            s, a, s_p, r, p[s][a][s_p][r]
                        );
                    }
                }
            }
        }

     */

        // en gros, p représente toutes nos combinaisons de state/action/next-step/reward possibles
        // quand c'est 0 c'est que cette combinaison est impossible, sinon c'est 1

        let env = Environment {
            S: S,
            A: A,
            R: R,
            T: T,
            p: p,
        };

        env
    }

    let mut env = linear_world();
    let len_S: usize = env.S.len();
    let len_A: usize = env.A.len();

    fn policy_iteration<TS: fmt::Debug,
                        TA: fmt::Debug,
                        TR: fmt::Debug,
                        TT: fmt::Debug,
                        TP: fmt::Debug>
                        (env: Environment<TS, TA, TR, TT, TP>, len_S: usize, len_A: usize, theta: f32, gamma: f32)
                        where TA: Iterator
    {

        let S = env.S;
        let A = env.A;
        let R = env.R;
        let T = env.T;
        let p = env.p;

        let mut rng = rand::thread_rng();
        let mut V: Vec<f32> = Vec::with_capacity(len_S);

        for _ in 0..len_S {
            V.push(rng.gen_range(0f32..1f32));
        }

        let mut Pi= Vec::with_capacity(len_S);

        for _ in 0..len_S {
            Pi.push(A.choose(&mut rand::thread_rng())); // mettre des valeurs aléatoires de A
        }


        println!("{:?}", Pi);


    }

    policy_iteration(env, len_S, len_A,0.00001, 0.999);

}
