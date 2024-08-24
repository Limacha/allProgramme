package fr.myapplication.fragments

import android.net.Uri
import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.ImageView
import android.widget.TextView
import androidx.fragment.app.Fragment
import androidx.recyclerview.widget.LinearLayoutManager
import androidx.recyclerview.widget.RecyclerView
import com.bumptech.glide.Glide
import com.google.gson.Gson
import com.google.gson.reflect.TypeToken
import fr.myapplication.R
import fr.myapplication.ReadActivity
import fr.myapplication.ScanModel
import fr.myapplication.adapter.ReadAdapter


class ReadFragement(
    private val context: ReadActivity,
    val scanname: String,
    val chapitre: Int
) : Fragment() {

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View? {
        val view = inflater.inflate(R.layout.fragment_home, container, false)

        //creez liste stock scan
        val scanList = arrayListOf<ScanModel>()

        // Ouvre le fichier JSON à partir des assets
        val inputStream = context.assets.open("scan.json")

        // Lit le contenu du fichier dans une chaîne
        val json = inputStream.bufferedReader().use { it.readText() }

        // Analyse le contenu JSON en tant qu'objet Kotlin
        val gson = Gson()
        val type = object : TypeToken<List<ScanModel>>() {}.type
        val data: MutableList<ScanModel> = gson.fromJson(json, type)

        // Ajoute chaque élément à la liste
        scanList.addAll(data)

        var result: ScanModel? = null

        for (scan in scanList) {
            if (scan.name.equals(scanname)) {
                result = scan
                break
            }
        }

        if (result != null) {
            System.out.println("Anime trouvé : " + result.name);
        } else {
            System.out.println("Aucun anime trouvé avec ce nom.");
        }

        println("adaptater")
        val collectionRecyclerView = view?.findViewById<RecyclerView>(R.id.vertical_recycler_view)
        println("adaptater")
        collectionRecyclerView?.adapter = ReadAdapter(context, result, R.layout.item_vertical_read, chapitre)
        collectionRecyclerView?.layoutManager = LinearLayoutManager(context)
        println("view")
        return view
    }
}