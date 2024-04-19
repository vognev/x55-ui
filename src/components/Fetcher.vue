<script setup lang="ts">
import { ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const id = ref("dQw4w9WgXcQ");
const msg = ref("");
const videos = reactive([] as string[]);

type YoutubeFormat = {
  quality: string,
  mimeType: string,
};

async function fetch() {
  msg.value = "Fetching ...";
  videos.length = 0;

  const data: string = await invoke("get_video_info", { id: id.value });

  const json = JSON.parse(data);
  console.log(json);

  msg.value = json?.videoDetails?.title ?? 'Untitled';

  // const formats = (json?.streamingData?.adaptiveFormats ?? []).concat(
  //   json.streamingData?.formats || []
  // );

  if (json?.videoDetails?.isLive) {
    videos.push(json?.streamingData?.dashManifestUrl);
    videos.push(json?.streamingData?.hlsManifestUrl);
  } else {
    const formats = json.streamingData?.formats || [];

    if (formats.length) {
      formats.sort((fmta: YoutubeFormat, fmtb: YoutubeFormat) => {
        let scorea = 0;
        scorea += (fmta.quality == "hd720" ? 2 : (fmta.quality == "large" ? 1 : 0));
        scorea += (fmta.mimeType.indexOf("mp4") != -1 ? 20 : 0)

        let scoreb = 0;
        scoreb += (fmtb.quality == "hd720" ? 2 : (fmtb.quality == "large" ? 1 : 0));
        scoreb += (fmtb.mimeType.indexOf("mp4") != -1 ? 20 : 0)

        return scoreb - scorea;
      });

      for (const format of formats) {
        if ('url' in format) {
          videos.push(format.url);
        }
      }
    }
  }

  console.log(videos)
}
</script>

<template>
  <form class="row" @submit.prevent="fetch">
    <input id="url" v-model="id" placeholder="videoId" />
    <button type="submit">Fetch</button>

  </form>

  <p>{{ msg }}</p>

  <video v-if="videos.length" style="margin: 0 auto" width="1280" height="720" autoplay="true" controls="true">
    <source :src="video" v-for="video of videos" />
  </video>
</template>
