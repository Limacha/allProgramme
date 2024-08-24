package fr.myapplication

import android.app.Dialog
import android.net.Uri
import android.os.Bundle
import android.view.ContextMenu
import android.view.View
import android.view.Window
import android.widget.ImageView
import android.widget.TextView
import androidx.fragment.app.Fragment
import androidx.recyclerview.widget.LinearLayoutManager
import androidx.recyclerview.widget.RecyclerView
import com.bumptech.glide.Glide
import fr.myapplication.adapter.DownloadAdapter
import fr.myapplication.adapter.ScanAdapter
import fr.myapplication.adapter.popupAdapter
import fr.myapplication.adapter.popupAdapterDownload

class ScanPopupDownload(
    private val adapter: DownloadAdapter,
    private val currentScan: ScanModel,
    val context: MainActivity
) : Dialog(adapter.context) {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        requestWindowFeature(Window.FEATURE_NO_TITLE)
        setContentView(R.layout.popup_scan_details)
        setupComponents()
        setupCloseButton()
        setupDownloadButton()
        setupStarButton()
        setupRecyclerView()
    }

    private fun setupStarButton() {
        val starButton = findViewById<ImageView>(R.id.star_button)

        //verif si like
        if (currentScan.liked) {
            starButton.setImageResource(R.drawable.ic_star)
        } else {
            starButton.setImageResource(R.drawable.ic_unstar)
        }

        starButton.setOnClickListener {
            currentScan.liked = !currentScan.liked
            if (currentScan.liked) {
                starButton.setImageResource(R.drawable.ic_star)
            } else {
                starButton.setImageResource(R.drawable.ic_unstar)
            }
        }
    }

    private fun setupDownloadButton() {
        val downloadButton = findViewById<ImageView>(R.id.download_button)

        downloadButton.setOnClickListener {
            dismiss()
        }
    }

    private fun setupCloseButton() {
        val closeButton = findViewById<ImageView>(R.id.close_button)

        closeButton.setOnClickListener {
            dismiss()
        }
    }

    private fun setupRecyclerView() {
        val recyclerView = findViewById<RecyclerView>(R.id.episode_recycler_list)
        recyclerView.layoutManager = LinearLayoutManager(context)
        recyclerView.adapter = popupAdapterDownload(context, currentScan)
    }

    private fun setupComponents() {
        val scanImage = findViewById<ImageView>(R.id.image_item)
        val scanName = findViewById<TextView>(R.id.popup_scan_name)
        val scanNb_ep = findViewById<TextView>(R.id.popup_scan_nb_episode)

        //charger image
        Glide.with(adapter.context).load(Uri.parse(currentScan.imageFile)).into(scanImage)

        //charge nom
        scanName.text = currentScan.name

        //charge nombre
        scanNb_ep.text = currentScan.nb_episode.toString().plus(" ").plus("episode")

    }
}