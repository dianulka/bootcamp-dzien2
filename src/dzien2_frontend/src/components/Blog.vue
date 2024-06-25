<template>
    <div>
        <button @click="pobierzWpisy">refresh</button>
        Cześć Blog! ^__^
        {{ wpisy }}
        <input v-model="nowyBlog" type="text">
        <button @click="dodajWpis">dodaj</button>

    </div>

</template>

<script> 
import { dzien2_backend } from "declarations/dzien2_backend/index"
export default {
    data() {
        return {
            wpisy : [], // lista
            nowyBlog: ""
        }
    },
    methods: {
        async pobierzWpisy() {
            this.wpisy = await dzien2_backend.odczytaj_wpisy()
        },
        async dodajWpis() {
            await dzien2_backend.dodaj_wpis(this.nowyBlog);
        }
    }
}
</script>