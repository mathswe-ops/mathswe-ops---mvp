// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause
// This file is part of https://github.com/mathsoftware/mathsoftware---mvp

import "./Ops.css";
import specialIcon from "@app/assets/special.png";
import { Heading } from "@app/main/Heading.tsx";

function Ops() {
    return <>
        <section className="swam">
            <div className="wrap">
                <Heading
                    id="special-ops"
                    title="Special Ops"
                    icon={ specialIcon }
                ></Heading>

                <p>Special Software and Models</p>

                <p>
                    MathSwe Ops automates the development and
                    deployment of mathematical software with <b>SWAM
                    (Special Software and Models)</b> operations.
                </p>

                <p>
                    Extended software required in real-world computing like,
                    operations, high-performance implementations, and
                    academic
                    tools are, examples of external domains for powering
                    modern
                    mathematics.
                </p>

                <p>
                    All the necessary tools extrinsic to mathematics are
                    aimed
                    to be equipped by <b>SWAM</b>.
                </p>
            </div>
        </section>
    </>;
}

export default Ops;
