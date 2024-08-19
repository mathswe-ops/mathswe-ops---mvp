// Copyright (c) 2024 Tobias Briones. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later
// This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp

import { JSX } from "react";

interface HeadingProps {
    id: string;
    title: string;
    icon: string;
}

interface SubHeadingProps {
    id: string;
    title: string;
    icon?: string;
}

interface SubSubHeadingProps {
    id: string;
    title: string;
}

function getHeadingClass(heading: JSX.Element) {
    const name = heading.type as string;

    if (name === "h2") {
        return "heading";
    }
    else {
        return "subheading";
    }
}

function HeadingCard(heading: JSX.Element, id: string, icon?: string) {
    const hClass = getHeadingClass(heading);
    const classes = `navbar-brand nav-link flex-fill p-0 ${ hClass }`;

    return <>
        <a
            className={ classes }
            href={ `#${ id }` }
        >
            { heading }
            { icon !== undefined &&
              <img
                  src={ icon }
                  alt="Math Software"
                  className="nav-icon"
              /> }
        </a>
    </>;
}

export function Heading({ id, title, icon }: HeadingProps) {
    return HeadingCard(<h2>{ title }</h2>, id, icon);
}

export function SubHeading({ id, title, icon }: SubHeadingProps) {
    return HeadingCard(<h3>{ title }</h3>, id, icon);
}

export function SubSubHeading({ id, title }: SubSubHeadingProps) {
    return HeadingCard(<h4>{ title }</h4>, id);
}
