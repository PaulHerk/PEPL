import { Box } from "@mui/joy";
import { createTheme } from "@mui/material/styles";

const theme = createTheme({
  status: {
    danger: "#e53e3e",
  },
  palette: {
    primary: {
      main: "#ec407a",
    },
    secondary: {
      main: "#ef9a9a",
    },
    neutral: {
      main: "#64748B",
      contrastText: "#fff",
    },
  },
});

declare module "@mui/material/styles" {
  interface Theme {
    status: {
      danger: React.CSSProperties["color"];
    };
  }

  interface Palette {
    neutral: Palette["primary"];
  }
  interface PaletteOptions {
    neutral: PaletteOptions["primary"];
  }

  interface PaletteColor {
    darker?: string;
  }
  interface SimplePaletteColorOptions {
    darker?: string;
  }
  interface ThemeOptions {
    status: {
      danger: React.CSSProperties["color"];
    };
  }
}
