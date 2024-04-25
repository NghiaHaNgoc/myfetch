use sysinfo::System;

const ARCH_LINUX: &str = "
                   -`                 
                  .o+`                
                 `ooo/                
                `+oooo:               
               `+oooooo:              
               -+oooooo+:             
             `/:-:++oooo+:            
            `/++++/+++++++:           
           `/++++++++++++++:          
          `/+++ooooooooooooo/`        
         ./ooosssso++osssssso+`       
        .oossssso-````/ossssss+`      
       -osssssso.      :ssssssso.     
      :osssssss/        osssso+++.    
     /ossssssss/        +ssssooo/-    
   `/ossssso+/:-        -:/+osssso+-  
  `+sso+:-`                 `.-/+oso: 
 `++:.                           `-/+/
 .`                                 `/
";

const NIXOS: &str = "
\x1b[34m          ::::.    \x1b[36m':::::     ::::'
\x1b[34m          ':::::    \x1b[36m':::::.  ::::'
\x1b[34m            :::::     \x1b[36m'::::.:::::
\x1b[34m      .......:::::..... \x1b[36m::::::::
\x1b[34m     ::::::::::::::::::. \x1b[36m::::::    \x1b[34m::::.
\x1b[34m    ::::::::::::::::::::: \x1b[36m:::::.  \x1b[34m.::::'
\x1b[36m           .....           ::::' \x1b[34m:::::'
\x1b[36m          :::::            '::' \x1b[34m:::::'
\x1b[36m ........:::::               ' \x1b[34m:::::::::::.
\x1b[36m:::::::::::::                 \x1b[34m:::::::::::::
\x1b[36m ::::::::::: \x1b[34m..              \x1b[34m:::::
\x1b[36m     .::::: \x1b[34m.:::            \x1b[34m:::::
\x1b[36m    .:::::  \x1b[34m:::::          \x1b[34m'''''    \x1b[36m.....
\x1b[36m    :::::   \x1b[34m':::::.  \x1b[36m......:::::::::::::'
\x1b[36m     :::     \x1b[34m::::::. \x1b[36m':::::::::::::::::'
\x1b[34m            .:::::::: \x1b[36m'::::::::::
\x1b[34m           .::::''::::.     \x1b[36m'::::.
\x1b[34m          .::::'   ::::.     \x1b[36m'::::.
\x1b[34m         .::::      ::::      \x1b[36m'::::.
";

pub fn logo() -> Option<String> {
    let os = System::name()?;
    match os.as_str() {
        "Arch Linux" => Some(String::from(ARCH_LINUX.trim())),
        "NixOS" => Some(NIXOS.trim().to_string()),
        _ => None,
    }
}
