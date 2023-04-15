use std::env::args;
use std::str::FromStr;

fn main() {
    let args = args().collect();
    args_programm(args); //unwrap lesson with args
}

fn args_programm(args: Vec<String>) {
    let x = args
        .iter() //iter gibt referenzen auf die Objekte zurück
        .nth(1)
        .map(|s: &String| {
            //deswegen muss hier nun eine & Referenz
            i32::from_str(s)
        })
        .unwrap_or(Ok(0))
        .unwrap_or_else(|err| {
            println!("Error while parsing: {}", err);
            0
        });

    for i in x..10 {
        println!("{i}");
    }
}

#[cfg(test)] //schiebt die ganzen tests in einen extra module, falls man eine library hat (möchte man nicht in der Produktion mitliefern)
mod test {
    use super::args_programm;

    #[test]
    fn test_my_programm() {
        let test_vec_list = ["foo", "1"]
        .into_iter()//gibt keine referenz an die objekte ansonsten hätten wir eine referenz auf referenz
        .map(|s| String::from(s))//map geht über die einzelnen objekte in der liste (iterativ) per iteration nimmt er das s in die closure und wird hier wird s dann zum String convertiert
        .collect(); //ohne collect würde nichts passieren, iter gibt hier nichts zurück.
        //let test_vec = vec![String::from("foo"), String::from("1")];
        args_programm(test_vec_list);
    }

    #[test]
    fn test_my_programm_withoutclosure() {

        let test_vec_list = ["foo", "1"]
        .into_iter()//gibt keine referenz an die objekte ansonsten hätten wir eine referenz auf referenz
        .map(String::from)//Falls nur eine Funktion von Closure aufgerufen wird braucht man die closure nicht (spart einen funktionsaufruf pro iteration)
        .collect(); //ohne collect würde nichts passieren, iter gibt hier nichts zurück.
        //let test_vec = vec![String::from("foo"), String::from("1")];
        args_programm(test_vec_list);
    }

    #[test]
    #[should_panic] //wenn man erwartet das der test das programm crashed
    fn test_my_programm_bad() {
        //gab keinen lol xD also der Fehlschlägt
        let test_vec = vec![String::from("foo"), String::from("")];
        args_programm(test_vec);
    }
}
