import type { PageServerLoad } from './$types'

export const load: PageServerLoad = async function (event) {
    return {
        // No blog data needed anymore - Features section is static
    }
}