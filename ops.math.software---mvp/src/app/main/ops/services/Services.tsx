// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

import "./Services.css";
import servicesIcon from "@app/assets/services.png";
import { Heading } from "@app/main/Heading.tsx";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { SubHeading } from "../../Heading.tsx";
import { faCloud } from "@fortawesome/free-solid-svg-icons";
import versionBadgeImg from "@app/assets/version-badge.png";
import projectBadgeImg from "@app/assets/project-badge.png";

interface InlineCodeProps {
    children: string;
}

function InlineCode({ children }: InlineCodeProps) {
    return <>
        <code className="code language-plaintext highlighter-rouge">
            { children }
        </code>
    </>;
}

interface CloudProps {
    method: string;
    endpoint: string;
    caption: string;
}

function Cloud({ method, endpoint, caption }: CloudProps) {
    return <>
        <figure>
            <div>
                <code className="d-block language-plaintext highlighter-rouge">
                    <FontAwesomeIcon
                        className="mx-2"
                        icon={ faCloud }
                        color="#fafafa"
                        style={ {
                            top: "4px",
                            background: "#212121",
                            padding: "4px 4px",
                            borderRadius: "4px",
                        } }
                        size="xs"
                    />
                    <span className="method">{ method }</span>
                    { endpoint }
                </code>
            </div>

            <figcaption>{ caption }</figcaption>
        </figure>
    </>;
}

function Services() {
    return <>
        <section className="bg-strip services">
            <div className="bg">
                <div className="wrap">
                    <Heading
                        id="services"
                        title="Services"
                        icon={ servicesIcon }
                    ></Heading>

                    <p>Web API for General-Purpose Engineering</p>

                    <p>
                        MathSwe Ops Services is a web application supporting
                        general-purpose software engineering needed for
                        mathematical software.
                    </p>

                    <SubHeading id="badge" title="Badge" />

                    <p>SVG Badges for Documentation</p>

                    <Cloud
                        method="GET"
                        endpoint="badge/version/:gitPlatform/:user/:repo"
                        caption="Version Badge"
                    />

                    <p>
                        The query <InlineCode>?path</InlineCode> defines a
                        subproject (e.g., a microservice).
                    </p>

                    <p>
                        <img src={ versionBadgeImg } alt="Version Badge" />
                    </p>

                    <Cloud
                        method="GET"
                        endpoint="badge/project/:project"
                        caption="Project Badge"
                    />

                    <p>
                        The query flag <InlineCode>?mvp</InlineCode> takes the
                        MVP version of the project.
                    </p>

                    <p>
                        <img src={ projectBadgeImg } alt="Project Badge" />
                    </p>

                    <p>
                        The <InlineCode>badge/project</InlineCode> endpoint is
                        exclusive to MathSwe projects.
                    </p>

                    <p><strong>MathSwe Ops Services v0.1.0</strong></p>

                    <p>
                        Technical documentation at&nbsp;
                        <a
                            href="https://github.com/mathswe-ops/services"
                            target="_blank"
                            rel="noreferrer"
                        >
                            MathSwe Ops Services | GitHub Repository
                        </a>.
                    </p>

                    <p>
                        MathSwe Ops Services is designated to satisfy
                        all <b>General SWE cloud requirements extrinsic to
                        MSW</b> that involve the development and deployment of
                        MSW.
                    </p>
                </div>
            </div>
        </section>
    </>;
}

export default Services;
