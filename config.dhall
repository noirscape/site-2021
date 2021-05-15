let defaultPort = env:PORT ? 4095

let Config =
      { Type =
          { port : Natural
          , resumeFname : Text
          }
      , default =
        { port = defaultPort
        , resumeFname = "./static/resume/resume.md"
        }
      }

in  Config::{
    , port = defaultPort
    }
