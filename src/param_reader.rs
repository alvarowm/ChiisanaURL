pub fn read_params(args: Vec<String>) -> String {
        if !args.is_empty() && args.len() >= 2 {
            for i in 1..args.len() {
                if args[i - 1].contains("-properties") {
                    return args[i].to_string();
                }
            }
        }
        return "".to_string();
    }

