import type { NextPage } from "next";
import Head from "next/head";
import Image from "next/image";
import styles from "../styles/Home.module.scss";
import init, { ErrorOnLexer, main } from "../pkg/pepl";
import { useEffect, useRef, useState } from "react";
import { Box, Grid } from "@mui/material";
// import "../styles/themes";
import { Alert, Textarea, Typography } from "@material-tailwind/react";

const Home: NextPage = () => {
  // const theme = useTheme();
  const [output, setOutput] = useState("Lexing...");
  const [code, setCode] = useState("");
  const isInitialMount = useRef(true);

  const helloWorldProgram =
    "!+0+21\\\n!+0+64\\\n!+0+6C\\\n!+0+72\\\n!+0+6F\\\n!+0+57\\\n!+0+20\\\n!+0+6F\\\n!+0+6C\\\n!+0+6C\\\n!+0+65\\\n!+0+48\\\n!+1+21\\\n>\\\n,0>\\\n?0:1\\\n<0\\\n?|\\\n!-0\\\n<";

  useEffect(() => {
    if (isInitialMount.current) {
      isInitialMount.current = false;
      setCode(helloWorldProgram);
      return;
    }
    init()
      .then(() => {
        setOutput(main(code));
        let outputNew = output.split("\n").map((value, index) => {
          <p>{value}</p>;
        });
        console.log(outputNew);
      })
      .catch((error) => {
        console.log(error);
        const errorType = error.error_on_lexer
          ? error.error_on_lexer
          : error.error_on_interpreter;
        console.log(errorType);

        setOutput(JSON.stringify(`${errorType.kind} ${errorType.position}`));
      });
  }, [code]);
  return (
    <div className={styles.container}>
      <Head>
        <title>Pepl demo</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main className={styles.main}>
        <a
          href="https://github.com/Kaenigsion/PEPL"
          style={{ marginBottom: 25 }}
        >
          <span className={styles.logo}>
            <Image src="/pepl.svg" alt="PEPL" width="360" height="80" />
          </span>
        </a>

        <Grid container justifyContent="space-between" flexWrap={"inherit"}>
          <Grid item style={{ marginRight: 20 }}>
            <Textarea
              label="You can start coding here!"
              className={styles.textarea}
              id="code"
              color="blue"
              defaultValue={helloWorldProgram}
              onChange={(value) => {
                setCode(value.target.value);
              }}
            />
          </Grid>
          <Grid item>
            <Alert variant="gradient" className={styles.alertBox}>
              <Typography variant="h4">{output}</Typography>
            </Alert>
          </Grid>
        </Grid>
      </main>

      <footer className={styles.footer}>
        <a
          href="https://vercel.com?utm_source=create-next-app&utm_medium=default-template&utm_campaign=create-next-app"
          target="_blank"
          rel="noopener noreferrer"
        >
          Powered by{" "}
          <span className={styles.logo}>
            <Image src="/vercel.svg" alt="Vercel Logo" width={72} height={16} />
          </span>
        </a>
      </footer>
    </div>
  );
};

export default Home;
