use std::fmt;

fn main() {
    println!("Hello, world!");

    #[derive(Debug)]
    struct Environment<TS, TA, TR, TT, TP> {
        S: TS,
        A: TA,
        R: TR,
        T: TT,
        p: TP,
    }

    fn linear_world() -> Environment<Vec<i8>,Vec<i8>,Vec<i8>,Vec<i8>,Vec<Vec<Vec<Vec<f32>>>>> {
        // CREATION DE L'ENVIRONNEMENT LINE WORLD
        // ensemble des états possibles
        let mut S: Vec<i8> = vec![0, 1, 2, 3, 4];
        // ensemble des actions possibles, O gauche, 1 droite
        let mut A: Vec<i8> = vec![0, 1];
        // ensemble des rewards possibles
        let mut R: Vec<i8> = vec![-1, 0, 1];
        // ensemble des états terminaux
        let mut T: Vec<i8> = vec![0, 4];
        // définition de p
        let mut p: Vec<Vec<Vec<Vec<f32>>>> = vec![
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

    fn policy_iteration<TS: std::fmt::Debug,
                        TA: std::fmt::Debug,
                        TR: std::fmt::Debug,
                        TT: std::fmt::Debug,
                        TP: std::fmt::Debug>
                        (env: Environment<TS, TA, TR, TT, TP>) {

        print!("{:?}", env);

    }

    policy_iteration(env);

}
