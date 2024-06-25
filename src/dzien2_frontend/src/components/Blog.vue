<template>
    <div>
        <button @click="pobierzWpisy">refresh</button>
        Cześć Blog! ^__^
        {{ wpisy }}
    </div>

</template>

<script> 
import { dzien2_backend } from "declarations/dzien2_backend/index"
export default {
    data() {
        return {
            wpisy : [] // lista

        }
    },
    methods: {
        async pobierzWpisy() {
            this.wpisy = await dzien2_backend.odczytaj_wpisy()
        }
    }
}
</script>