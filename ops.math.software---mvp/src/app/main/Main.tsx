// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause
// This file is part of https://github.com/mathsoftware/mathsoftware---mvp

import Header from "@app/main/Header.tsx";
import "@app/main/Main.css";
import Ops from "./ops/Ops.tsx";

function Main() {
    return <>
        <section>
            <main>
                <article>
                    <section id="ops">
                        <Header></Header>

                        <p>Mathematical Software Operations</p>
                    </section>

                    <Ops />
                </article>
            </main>
        </section>
    </>;
}

export default Main;
