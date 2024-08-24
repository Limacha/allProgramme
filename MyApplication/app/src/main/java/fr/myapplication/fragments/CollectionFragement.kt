package fr.myapplication.fragments

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import androidx.recyclerview.widget.LinearLayoutManager
import androidx.recyclerview.widget.RecyclerView
import com.google.gson.Gson
import com.google.gson.reflect.TypeToken
import fr.myapplication.MainActivity
import fr.myapplication.R
import fr.myapplication.ScanModel
import fr.myapplication.adapter.ScanAdapter
import org.json.JSONObject
import java.io.File
import java.io.FileOutputStream
import java.io.OutputStreamWriter


class CollectionFragement(
    private val context: MainActivity
) : Fragment() {

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View? {
        val view = inflater.inflate(R.layout.fragment_collection, container, false)

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

        val collectionRecyclerView = view?.findViewById<RecyclerView>(R.id.collection_recycler_list)
        collectionRecyclerView?.adapter = ScanAdapter(
            context,
            scanList.filter { it.liked },
            R.layout.item_vertical_scan_collection
        )
        collectionRecyclerView?.layoutManager = LinearLayoutManager(context)

        return view
    }
}