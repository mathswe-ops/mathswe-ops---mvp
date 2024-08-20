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
                    id="swam"
                    title="SWAM"
                    icon={ specialIcon }
                ></Heading>

                <p>Special Software and Models</p>

                <p>
                    While mathematical software requires a <b>DSL (Domain
                    Specific Language)</b> to be engineering grade, these
                    formalities require non-mathematical <b>Special
                    Operations</b> to reach production.
                </p>

                <p>
                    Implementation details can encompass development and
                    deployment operations, compilation, high-performance
                    implementations, and academic tools.
                </p>

                <p>
                    SWAM is an abstraction based
                    on <b>MSWE (Mathematical Software Engineering)</b>, which
                    ensures the <b>integration</b> of mathematical formalities
                    into production. Therefore, the expression <i>DSL +
                    SWAM</i> partitions an engineering-grade software design.
                </p>

                <p>
                    MSWE empowers SWAM implementations to equip all the tools
                    and operations <b>extrinsic to mathematics</b> to
                    materialize mathematical software.
                </p>
            </div>
        </section>
    </>;
}

export default Ops;
