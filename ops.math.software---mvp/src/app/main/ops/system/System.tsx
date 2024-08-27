// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

import "./System.css";
import systemIcon from "@app/assets/system.png";
import { Heading } from "@app/main/Heading.tsx";
import React from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faTerminal } from "@fortawesome/free-solid-svg-icons/faTerminal";
import { SubHeading, SubSubHeading } from "../../Heading.tsx";

const ImageGrid: React.FC = () => {
    const serverImages = [
        "Rust",
        "Go",
        "Sdkman",
        "Java",
        "Gradle",
        "Nvm",
        "Node",
        "Miniconda",
    ];
    const desktopImages = ["Zoom", "VsCode", "JetBrainsToolbox"];
    const jetBrainsIdeImages = [
        "IntelliJIdea",
        "WebStorm",
        "RustRover",
        "CLion",
        "PyCharm",
        "DataGrip",
        "Goland",
        "Rider",
        "PhpStorm",
        "RubyMine",
    ];

    return (
        <div className="image-grid-container">
            <div className="title">
                <strong>Server</strong>
            </div>

            <div className="grid">
                { serverImages.map((image, index) => (
                    <div className="box" key={ index }>
                        { image }
                    </div>
                )) }
            </div>

            <div className="title">
                <strong>Desktop</strong>
            </div>

            <div className="grid">
                { desktopImages.map((image, index) => (
                    <div className="box" key={ index }>
                        { image }
                    </div>
                )) }
            </div>

            <div className="title">
                <strong>JetBrains IDE (Toolbox)</strong>
            </div>

            <div className="grid">
                { jetBrainsIdeImages.map((image, index) => (
                    <div className="box" key={ index }>
                        { image }
                    </div>
                )) }
            </div>
        </div>
    );
};

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

interface CommandProps {
    command: string;
    caption: string;
}

function Command({ command, caption }: CommandProps) {
    return <>
        <figure>
            <div>
                <code className="d-block language-plaintext highlighter-rouge">
                    <FontAwesomeIcon
                        className="mx-2"
                        icon={ faTerminal }
                        color="#fafafa"
                        style={ {
                            top: "2px",
                            background: "#212121",
                            padding: "2px 4px",
                            borderRadius: "4px",
                        } }
                        size="xs"
                    />
                    { command }
                </code>
            </div>

            <figcaption>{ caption }</figcaption>
        </figure>
    </>;
}

function System() {
    return <>
        <section className="bg-strip system">
            <div className="bg">
                <div className="wrap">
                    <Heading
                        id="system"
                        title="System"
                        icon={ systemIcon }
                    ></Heading>

                    <p>OS Software Operations</p>

                    <p>
                        MathSwe System Ops automates software operations in
                        Linux, such as installation, uninstallation, and
                        configuration, to allow you to set up server VMs and
                        desktop Workstations by running a command.
                    </p>

                    <SubHeading id="image-ops" title="Image Ops" />

                    <p>Install + Uninstall</p>

                    <Command
                        command="system install { image_1, image_2, ..., image_n}"
                        caption="Installation"
                    />

                    <p>
                        The flag <InlineCode>--config</InlineCode> provides
                        image restoration.

                        <p>
                            For
                            example, <InlineCode>system install --config
                            miniconda</InlineCode>.
                        </p>
                    </p>

                    <Command
                        command="system uninstall { image_1, image_2, ..., image_n}"
                        caption="Uninstallation"
                    />

                    <Command
                        command="system reinstall { image_1, image_2, ..., image_n}"
                        caption="Reinstallation"
                    />

                    <Command
                        command="system config { image_1, image_2, ..., image_n}"
                        caption="Configuration"
                    />

                    <SubSubHeading
                        id="available-images"
                        title="Available Images"
                    />

                    <ImageGrid />

                    <p><strong>MathSwe System Ops MVP v0.1.0</strong></p>

                    <p>
                        Technical documentation at&nbsp;
                        <a
                            href="https://github.com/mathswe-ops/mathswe-ops---mvp/tree/main/system"
                            target="_blank"
                            rel="noreferrer"
                        >
                            MathSwe System Ops MVP | GitHub Repository
                        </a>.
                    </p>

                    <p>
                        System is a CLI application with a reliable and
                        evolving design that <b>makes cloud VMs and desktop work
                        machines productive from cold OS installation</b>. It
                        automates cold-start DevOps and staff onboarding as per
                        your
                        organization&apos;s standards.
                    </p>
                </div>
            </div>
        </section>
    </>;
}

export default System;
