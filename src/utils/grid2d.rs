//
// Réécriture de grid (array 2D) en utilisant les vecteurs
// pour ne pas devoir fixer de taille en dur
//

// use crate::utils::pause;

// Au dessus de MAX_AFFICHAGE, print() ne fait rien
const MAX_AFFICHAGE: usize = 99;

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Grid2D {
    pub max_l:usize,
    pub max_c:usize,
    pub grid:Vec<Vec<char>>,
}

impl Grid2D {
    pub fn new(input:&str) -> Grid2D {
        let mut grid = Vec::new();
        let max_l = input.lines().count(); // nombre de lignes
        let max_c = input.lines().next().unwrap().chars().count(); // taille de la première ligne
    
        input.lines().enumerate().for_each(|(l,line)| {
            if line.chars().count() != max_c {
                panic!("** ERREUR : Ce n'est pas une grille : la ligne {} n'a pas la même longueur que la ligne 1",l+1);
            } else {
                let mut vec_l = Vec::new();
                    line.chars().for_each(|ch| {vec_l.push(ch);});
                    grid.push(vec_l);        
            }
            });
            Grid2D { max_l, max_c,grid}
        }
    
    /// 
    /// Crée une grille de max_l x max_c avec le char ch
    /// 
    pub fn new_empty(max_l:usize, max_c:usize,ch:char) -> Grid2D {
        let mut grid = Vec::new();

        for _ in 0..max_l {
            let mut vec_l = Vec::new();
            for _ in 0..max_c {
                vec_l.push(ch);
            }
            grid.push(vec_l);        
        }                
        Grid2D { max_l, max_c,grid}
    }

    ///
    /// Accepte une position et un mouvement
    /// Renvoie true si pos+mvt est dans les limites de la grille
    /// 
    pub fn is_next_valid(&self,pos:(usize,usize),mvt:(isize,isize)) -> Option<(usize,usize)> {
        let l = pos.0 as isize + mvt.0;
        let c = pos.1 as isize + mvt.1;
        if l >= 0 && l< self.max_l as isize &&
        c >= 0 && c < self.max_c as isize {
            return Some((l as usize ,c as usize));
        }
        None
    }

    ///
    /// Affiche la grille avec numéros de lignes et de colonnes
    /// 
    pub fn print(&self){
        if self.max_l > MAX_AFFICHAGE || self.max_c > MAX_AFFICHAGE {
            println!(
                "** WARNING : la grille fait plus que {} x {} => pas d'affichage",
                MAX_AFFICHAGE, MAX_AFFICHAGE
            );
            return;
        }
        // num de colonnes
        print!(" C-> ");
        for c in 0..self.max_c { print!(" {:#02}", c); }
        println!();
        
        // cadre supérieur
        print!("    ┌");
        for _ in 0..self.max_c { print!("───"); }
        println!("─┐");

        // lignes + cadre + une ligne de la grille
        //for l in 0..MAX_GRID_L {
        self.grid.iter().enumerate().for_each(|(l,line)| {
            print!("L{:#02} │", l);
            line.iter().for_each(|ch| {
                print!("  {}", ch);
            });
            println!(" │");            
        });
        
        // bord inférieur
        print!("    └");
        for _ in 0..self.max_c { print!("───"); }
        println!("─┘");
    }

        ///
    /// Affiche une partie de la grille avec numéros de lignes et de colonnes
    /// 
    pub fn print_sub(&self,l_min:usize,l_max:usize,c_min:usize,c_max:usize){
        if l_max-l_min > MAX_AFFICHAGE || c_max-c_min > MAX_AFFICHAGE {
            println!(
                "** WARNING : la sous-grille fait plus que {} x {} => pas d'affichage",
                MAX_AFFICHAGE, MAX_AFFICHAGE
            );
            return;
        }
        // num de colonnes
        println!(" C-> {} to {}",c_min,c_max);
        // for c in c_min..c_max { print!(" {:#02}", c); }
        // println!();
        
        // cadre supérieur
        print!("    ┌");
        for _ in c_min..c_max { print!("───"); }
        println!("─┐");

        // lignes + cadre + une ligne de la grille
        //for l in 0..MAX_GRID_L {
        self.grid.iter().enumerate().for_each(|(l,line)| {
            if l>=l_min && l<l_max {
                print!("L{:#02} │", l);
                line.iter().enumerate().for_each(|(c,ch)| {
                    if c>= c_min && c<c_max {
                        print!("  {}", ch);
                    }
                });
                println!(" │");         
            }      
        });
        
        // bord inférieur
        print!("    └");
        for _ in c_min..c_max { print!("───"); }
        println!("─┘");
    }

    ///
    /// Affiche la grille avec numéros de lignes et de colonnes
    /// et les points d'un vecteur de points
    /// 
    pub fn print_with_vec(&self,v :&[(usize,usize)],display_char:char){
        if self.max_l > MAX_AFFICHAGE || self.max_c > MAX_AFFICHAGE {
            println!(
                "** WARNING : la grille fait plus que {} x {} => pas d'affichage",
                MAX_AFFICHAGE, MAX_AFFICHAGE
            );
            return;
        }
        // num de colonnes
        print!(" C-> ");
        for c in 0..self.max_c { print!(" {:#02}", c); }
        println!();
        
        // cadre supérieur
        print!("    ┌");
        for _ in 0..self.max_c { print!("───"); }
        println!("─┐");

        // lignes + cadre + une ligne de la grille
        //for l in 0..MAX_GRID_L {
        self.grid.iter().enumerate().for_each(|(l,line)| {
            print!("L{:#02} │", l);
            line.iter().enumerate().for_each(|(c,ch)| {
                if v.contains(&(l,c)) {
                    print!("  {}",display_char);
                } else {
                    print!("  {}", ch);
                }
            });
            println!(" │");
        });
        
        // bord inférieur
        print!("    └");
        for _ in 0..self.max_c { print!("───"); }
        println!("─┘");
    }

    ///
    /// Renvoie le caractère à un point donné
    /// 
    pub fn get_at(&self,pt:(usize,usize)) -> char {
        self.grid[pt.0][pt.1]
    }

    ///
    /// Modifie le caractère à un point donné
    /// 
    pub fn set_at(&mut self,pt:(usize,usize),value:char) {
        self.grid[pt.0][pt.1] = value;
    }

    ///
    /// Renvoie le nombre d'occurence du caractère cc dans la grille
    ///
    pub fn count_occurences(&self,cc:char) -> usize {
        self.grid.iter().flatten().filter(|&c| {*c==cc}).count()
    }

    ///
    /// Renvoie le nombre d'occurence du caractère cc dans une ligne
    ///
    pub fn count_occurences_in_line(&self,cc:char,line:usize) -> usize {
        self.grid[line].iter().filter(|&c| {*c==cc}).count()
    }

    ///
    /// Renvoie le nombre d'occurence du caractère cc dans une colonne
    ///
    pub fn count_occurences_in_column(&self,cc:char,col:usize) -> usize {
        let mut count = 0;
        for l in 0..self.max_l {
            if self.grid[l][col] == cc {
                count += 1;
            }
        }
        count
    }


    ///
    /// Renvoie un vecteur des coordonnées d'un caractère
    ///
    pub fn get_vec_of_char_positions( &self,cc: char ) -> Vec<(usize, usize)> {
        let mut vec = Vec::new();

        self.grid.iter().enumerate().for_each(|(l,line)| {
            line.iter().enumerate().for_each(|(c,ch)| {
                if ch.eq(&cc) {
                    vec.push((l, c));
                }
            })
        });
        vec
    }

    ///
    /// Renvoie la première occurence des coordonnées d'un caractère
    ///
    pub fn get_char_position( &self,cc: char ) -> Option<(usize,usize)> {

        for l in 0..self.max_l {
            for c in 0..self.max_c {
                if self.get_at((l,c)).eq(&cc) {
                    return Some((l,c));
                }
            }
        }
        None        
    }

    ///
    /// Renvoie un vecteur des caractères des 8 cases adjacentes
    /// (gauche, droite, haut, bas et diagonales)
    ///
    pub fn get_adjacents(&self, l: usize, c: usize ) -> Vec<(usize, usize, char)> {
        let mut ret = Vec::new();
        let adj: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
    
        for (dl, dc) in adj {
            if (dl < 0 && l == 0) ||
               (dc < 0 && c == 0 ) ||
               (dl > 0 && l+1 == self.max_l) ||
               (dc > 0 && c+1 == self.max_c )
            {} else {
                let new_l = (dl + (l as i32)) as usize;
                let new_c = (dc + (c as i32)) as usize;
                ret.push((new_l, new_c, self.grid[new_l][new_c]));
            }
        }
        ret
    }

    ///
    /// Renvoie un vecteur des caractères des 4 cases adjacentes
    /// (haut, gauche, droite, bas)
    ///
    pub fn get_adjacents_ortho(&self, l: usize, c: usize ) -> Vec<(usize, usize, char)> {
        let mut ret = Vec::new();
        let adj: [(i32, i32); 4] = [
            (0, 1),
            (-1, 0),
            (0, -1),
            (1, 0),
        ];
    
        for (dl, dc) in adj {
            if (dl < 0 && l == 0) ||
               (dc < 0 && c == 0 ) ||
               (dl > 0 && l+1 == self.max_l) ||
               (dc > 0 && c+1 == self.max_c )
            {} else {
                let new_l = (dl + (l as i32)) as usize;
                let new_c = (dc + (c as i32)) as usize;
                ret.push((new_l, new_c, self.grid[new_l][new_c]));
            }
        }
        ret
    }

    ///
    /// rotate the grid 90° counter-clockwise
    /// 
    #[allow(clippy::needless_range_loop)]
    pub fn rotate(self) -> Grid2D {
        let mut new_grid : Vec<Vec<char>> = (0..self.max_c).map(|_| Vec::new()).collect();
        for l in 0..self.max_l {
            for c in 0..self.max_c {
                new_grid[self.max_c-c-1].push( self.grid[l][c]);
            }
        }
        Grid2D {max_l:self.max_c, max_c:self.max_l, grid:new_grid}
    }
}
