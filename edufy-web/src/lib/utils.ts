import {
    SITE_DESCRIPTION,
    SITE_IMAGE,
    SITE_TITLE,
    SITE_URL,
} from '$lib/constants';

import type { Cookies } from '@sveltejs/kit'
import cookie from 'cookie'

export const waitVisible = (el: string, callBack: (element: Element) => void) => {
    window.setTimeout(function () {
        var element = document.getElementsByClassName(el)[0];
        if (element) {
            // console.log('visible:', false);
            window.getComputedStyle(element).display === 'block' ? waitVisible(el, callBack) : callBack(element);
        } else {
            waitVisible(el, callBack);
        }
    }, 200)
}

export const changeUrlParamsWithoutRefresh = (term: string, facetValueIds: string[]) => {
    const f = facetValueIds.join('-');
    return window.history.pushState(
        '',
        '',
        `${window.location.origin}${window.location.pathname}?q=${term}${f ? `&f=${f}` : ''}`
    );
};

export const generateDocumentHead = (
    url = SITE_URL,
    title = SITE_TITLE,
    description = SITE_DESCRIPTION,
    image = SITE_IMAGE
) => {
    const OG_METATAGS = [
        { property: 'og:type', content: 'website' },
        { property: 'og:url', content: url },
        { property: 'og:title', content: title },
        {
            property: 'og:description',
            content: description,
        },
        {
            property: 'og:image',
            content: image ? image + '?w=800&h=800&format=webp' : undefined,
        },
    ];
    const TWITTER_METATAGS = [
        { property: 'twitter:card', content: 'summary_large_image' },
        { property: 'twitter:url', content: url },
        { property: 'twitter:title', content: title },
        {
            property: 'twitter:description',
            content: description,
        },
        {
            property: 'twitter:image',
            content: image ? image + '?w=800&h=800&format=webp' : undefined,
        },
    ];
    return { title, meta: [...OG_METATAGS, ...TWITTER_METATAGS] };
};

export const parseAuthCookie = async function (setCookie: string[] = [], cookies: Cookies) {
    if (!setCookie) return
    try {
        let locals = { XSRF:'', token:'' }
        for (let rawCookie of setCookie) {
            let parsedCookie = cookie.parse(rawCookie)
            console.log(parsedCookie)
            if ('XSRF-TOKEN' in parsedCookie) {
                cookies.set('XSRF-TOKEN', parsedCookie['XSRF-TOKEN'], {
                    path: '/',
                    expires: new Date(parsedCookie['expires']),
                    sameSite: 'strict',
                    httpOnly: true,
                    secure: true
                })
                locals.XSRF = parsedCookie['XSRF-TOKEN']
            }
            if ('schoolify_session' in parsedCookie) {
                cookies.set('schoolify_session', parsedCookie['schoolify_session'], {
                    path: '/',
                    expires: new Date(parsedCookie['expires']),
                    sameSite: 'strict',
                    httpOnly: true,
                    secure: true
                })
                locals.token = parsedCookie['schoolify_session']
            }
        }
        return locals
    } catch (e) {
        console.log(e)
        return
    }
}