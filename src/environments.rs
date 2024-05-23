fn line_world() -> (Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>, Vec<Vec<Vec<Vec<f32>>>>) {
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

    return (S, A, R, T, p);
}

fn grid_world() {




}