package fr.myapplication

import java.util.Dictionary

class ScanModel(
    val name: String = "Name",
    var nb_episode: Int = 0,
    val last_chap: Int = 1,
    var img_epi: Map<String, Int> = mapOf(),
    val imageFile: String = "",
    var liked: Boolean = false,
    var download: Boolean = false
)