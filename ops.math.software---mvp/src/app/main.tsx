// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

import ReactDOM from "react-dom/client";
import "./index.css";
import App from "@app/App.tsx";
import "@app/assets/msw-engineer.css";
import MathJaxContext from "better-react-mathjax/MathJaxContext";
import React from "react";

const mathJaxConfig = {
    loader: {load: ["input/asciimath"]},
};

ReactDOM
    .createRoot(document.getElementById("root")!)
    .render(
        <React.StrictMode>
            <MathJaxContext config={ mathJaxConfig }>
                <App />
            </MathJaxContext>
        </React.StrictMode>,
    );
