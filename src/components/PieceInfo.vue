`<template>
  <div class="information-container piece-info">
    <h3>{{ workTitle }}</h3>
    <div class="composer-info">
      <h4>Composer: <span class="composer">{{ composer }}</span></h4>
      <h4>Arranger: <span class="arranger">{{ arranger }}</span></h4>
    </div>
  </div>
</template>

<script>
export default {
  name: 'PieceInfo',
  props: {
    workTitle: {
      type: String,
      required: true
    },
    composer: {
      type: String,
      required: true
    },
    arranger: {
      type: String,
      required: true
    }
  }
}
</script>

<style scoped>
.piece-info {
  background: linear-gradient(135deg, #4CAF50 0%, #45a049 100%);
  color: white;
  padding: 20px;
  border-radius: 8px;
}

.piece-info h3 {
  color: white;
  border-bottom: 1px solid rgba(255, 255, 255, 0.3);
  font-size: 1.6em;
  margin-bottom: 15px;
}

.composer-info {
  display: flex;
  gap: 20px;
  flex-wrap: wrap;
}

.composer-info h4 {
  color: rgba(255, 255, 255, 0.9);
  margin: 0;
}

.composer-info span {
  font-weight: 600;
  color: white;
}
</style>`
