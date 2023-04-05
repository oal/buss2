<template>
  <div>
    Journey
    <pre>
   {{ journey }}
 </pre
    >
  </div>
</template>

<script>
export default {
  name: 'JourneyPage',
  created() {
    this.loadData();
  },
  data() {
    return {
      journey: null,
    };
  },
  methods: {
    loadData() {
      this.$axios
        .get(`/api/journeys/${this.$route.params.id}`)
        .then((response) => {
          this.journey = response.data;
        })
        .catch((error) => {
          console.log(error);
        });
    },
  },
};
</script>

<style lang="scss" scoped></style>
