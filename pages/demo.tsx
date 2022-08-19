import type { NextPage } from "next";
import Head from "next/head";
import Image from "next/image";
import styles from "../styles/Home.module.scss";
import init, { ErrorOnLexer, main } from "../pkg/pepl";
import { Component, useEffect, useRef, useState } from "react";
import { Box, TextField } from "@mui/joy";
import "../styles/themes";
import { useTheme } from "@mui/material";

const Home: NextPage = () => {
  const theme = useTheme();
  const [output, setOutput] = useState("Lexing...");
  const [code, setCode] = useState("");
  const isInitialMount = useRef(true);

  const helloWorldProgram =
    "!+0+21\\!+0+64\\!+0+6C\\!+0+72\\!+0+6F\\!+0+57\\!+0+20\\!+0+6F\\!+0+6C\\!+0+6C\\!+0+65\\!+0+48\\!+1+21\\>\\,0>\\?0:1\\<0\\?|\\!-0\\<";

  useEffect(() => {
    if (isInitialMount.current) {
      isInitialMount.current = false;
      setCode(helloWorldProgram);
      return;
    }
    init()
      .then(() => setOutput(main(code)))
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
        <h1 className={styles.title}>Welcome to</h1>
        <a href="https://github.com/Kaenigsion/PEPL">
          <span className={styles.logo}>
            <Image src="/pepl.svg" alt="PEPL" width="720" height="160" />
          </span>
        </a>

        <div>
          <Box
            sx={{
              width: 900,
              margin: 5,
              py: 1,
              display: "flex",
              flexDirection: "row",
              gap: 2,
              alignItems: "center",
            }}
          >
            <Box sx={{ width: 500 }}>
              <TextField
                fullWidth
                autoComplete="off"
                placeholder="Your program!"
                label="You can start coding here!"
                size="lg"
                id="code"
                defaultValue={helloWorldProgram}
                onChange={(value) => {
                  setCode(value.target.value);
                }}
              />
            </Box>
            <div id="output">
              <p>
                The output: <code className={styles.code}>{output}</code>
              </p>
            </div>
          </Box>
        </div>
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
