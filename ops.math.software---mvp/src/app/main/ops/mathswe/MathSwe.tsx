// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

import "./MathSwe.css";
import mathsweIcon from "@app/assets/mathswe.svg";
import { Heading } from "@app/main/Heading.tsx";
import { SubHeading } from "../../Heading.tsx";
import specialIcon from "@app/assets/special.png";

function MathSwe() {
    return <>
        <section className="bg-strip mathswe">
            <div className="bg">
                <div className="wrap">
                    <Heading
                        id="mathswe"
                        title="MathSwe"
                        icon={ mathsweIcon }
                    ></Heading>

                    <p>Supporting Modern Mathematics</p>

                    <p>
                        While mathematical software must be open source to be
                        engineering-grade, it demands support when running
                        real-world operations.
                    </p>

                    <p>
                        The organization&apos;s vision encompasses an OSS
                        community, connecting with talent and business
                        operations.
                    </p>

                    <p>
                        Although MSW must be open source, business software
                        is limited to the production-grade rather than the
                        engineering one and <i>sometimes</i> may be
                        closed-source. They can encompass
                        esoteric
                        SWAM
                        integrating third-party standards, formats,
                        customer requirements, or business needs.
                    </p>

                    <p>
                        MathSwe belongs to the most concrete MSWE spectrum with
                        challenges involving extrinsic software, talent
                        networking, and transparent entrepreneurial undertakings
                        to <b>deliver modern mathematics</b>.
                    </p>

                    <SubHeading
                        id="mathswe-ops"
                        title="MathSwe Ops"
                        icon={ specialIcon }
                    />

                    <p>
                        Automating MSW Development and Deployment
                    </p>

                    <p>
                        Currently providing MVP tools to automate Ubuntu.
                    </p>

                    <SubHeading
                        id="mathswe-com"
                        title="MathSwe Com"
                        icon={ mathsweIcon }
                    />

                    <p>
                        MathSwe Business Operations
                    </p>

                    <p>In progress and forthcoming...</p>
                </div>
            </div>
        </section>
    </>;
}

export default MathSwe;
